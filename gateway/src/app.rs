use anyhow::{Context, anyhow};
use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use mongodb::{
    Client,
    bson::{Bson, DateTime, doc, oid::ObjectId},
    results::CollectionSpecification,
};
use serde::Serialize;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::task::yield_now;

use crate::GatewaySettings;
use crate::MongoSettings;
use crate::tasks::{
    CreateTaskInput, TASK_STATUS_PENDING, TASKS_COLLECTION_NAME, TaskValidationError,
    tasks_collection_is_prepared, validate_create_task_payload,
};

#[derive(Clone)]
struct AppState {
    mongo_client: Arc<SharedMongoClient>,
}

struct SharedMongoClient {
    settings: MongoSettings,
    state: Mutex<MongoClientState>,
}

enum MongoClientState {
    Uninitialized,
    Initializing,
    Ready(Client),
}

impl SharedMongoClient {
    fn new(settings: MongoSettings) -> Self {
        Self {
            settings,
            state: Mutex::new(MongoClientState::Uninitialized),
        }
    }

    async fn database(&self) -> anyhow::Result<mongodb::Database> {
        let client = self.client().await?;
        Ok(client.database(&self.settings.database_name))
    }

    async fn client(&self) -> anyhow::Result<Client> {
        loop {
            let should_initialize = {
                let mut state = self.lock_state()?;

                match &*state {
                    MongoClientState::Ready(client) => return Ok(client.clone()),
                    MongoClientState::Uninitialized => {
                        *state = MongoClientState::Initializing;
                        true
                    }
                    MongoClientState::Initializing => false,
                }
            };

            if should_initialize {
                return self.initialize_client().await;
            }

            yield_now().await;
        }
    }

    async fn initialize_client(&self) -> anyhow::Result<Client> {
        match Client::with_uri_str(&self.settings.uri).await {
            Ok(client) => {
                let mut state = self.lock_state()?;
                *state = MongoClientState::Ready(client.clone());
                Ok(client)
            }
            Err(error) => {
                let mut state = self.lock_state()?;
                *state = MongoClientState::Uninitialized;
                Err(error).context("failed to create MongoDB client")
            }
        }
    }

    fn lock_state(&self) -> anyhow::Result<MutexGuard<'_, MongoClientState>> {
        self.state
            .lock()
            .map_err(|_| anyhow!("mongo client state lock poisoned"))
    }
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: ErrorResponse,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    details: Vec<crate::tasks::ValidationDetail>,
}

#[derive(Debug, Serialize)]
struct CreateTaskResponse {
    id: String,
    title: String,
    status: &'static str,
    created_at: String,
}

#[derive(Debug)]
struct PersistedTask {
    id: ObjectId,
    title: String,
    created_at: DateTime,
}

pub fn build_router(settings: GatewaySettings) -> Router {
    let state = AppState {
        mongo_client: Arc::new(SharedMongoClient::new(settings.mongo)),
    };

    Router::new()
        .route("/healthz", get(healthcheck))
        .route("/tasks", post(post_tasks))
        .with_state(state)
}

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

async fn post_tasks(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    if !has_json_content_type(&headers) {
        return bad_request_response("request content-type must be application/json");
    }

    let payload = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return bad_request_response("request body must be valid JSON"),
    };

    let input = match validate_create_task_payload(payload) {
        Ok(input) => input,
        Err(error) => return validation_error_response(error),
    };

    match create_task(&state, input).await {
        Ok(task) => created_response(task),
        Err(_) => persistence_error_response(),
    }
}

async fn create_task(state: &AppState, input: CreateTaskInput) -> anyhow::Result<PersistedTask> {
    let database = state.mongo_client.database().await?;
    ensure_tasks_collection_is_prepared(&database).await?;

    let created_at = DateTime::now();
    let insert_result = database
        .collection::<mongodb::bson::Document>(TASKS_COLLECTION_NAME)
        .insert_one(doc! {
            "title": &input.title,
            "status": TASK_STATUS_PENDING,
            "created_at": created_at,
        })
        .await?;

    let id = match insert_result.inserted_id {
        Bson::ObjectId(id) => id,
        other => anyhow::bail!("MongoDB returned a non-ObjectId inserted_id: {other:?}"),
    };

    Ok(PersistedTask {
        id,
        title: input.title,
        created_at,
    })
}

async fn ensure_tasks_collection_is_prepared(database: &mongodb::Database) -> anyhow::Result<()> {
    let mut cursor = database
        .list_collections()
        .filter(doc! { "name": TASKS_COLLECTION_NAME })
        .await?;

    if !cursor.advance().await? {
        anyhow::bail!("tasks collection is not prepared");
    }

    let specification: CollectionSpecification = cursor.deserialize_current()?;

    if !tasks_collection_is_prepared(&specification) {
        anyhow::bail!("tasks collection validator is not prepared for the foundation contract");
    }

    Ok(())
}

fn has_json_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .is_some_and(|value| value.eq_ignore_ascii_case("application/json"))
}

fn created_response(task: PersistedTask) -> Response {
    match task.created_at.try_to_rfc3339_string() {
        Ok(created_at) => (
            StatusCode::CREATED,
            axum::Json(CreateTaskResponse {
                id: task.id.to_hex(),
                title: task.title,
                status: TASK_STATUS_PENDING,
                created_at,
            }),
        )
            .into_response(),
        Err(_) => persistence_error_response(),
    }
}

fn bad_request_response(message: &'static str) -> Response {
    error_response(StatusCode::BAD_REQUEST, "BAD_REQUEST", message, Vec::new())
}

fn validation_error_response(error: TaskValidationError) -> Response {
    error_response(
        StatusCode::UNPROCESSABLE_ENTITY,
        "TASK_VALIDATION_FAILED",
        error.message,
        error.details,
    )
}

fn persistence_error_response() -> Response {
    error_response(
        StatusCode::SERVICE_UNAVAILABLE,
        "TASK_PERSISTENCE_FAILED",
        "task could not be persisted",
        Vec::new(),
    )
}

fn error_response(
    status: StatusCode,
    code: &'static str,
    message: &'static str,
    details: Vec<crate::tasks::ValidationDetail>,
) -> Response {
    (
        status,
        axum::Json(ErrorBody {
            error: ErrorResponse {
                code,
                message,
                details,
            },
        }),
    )
        .into_response()
}
