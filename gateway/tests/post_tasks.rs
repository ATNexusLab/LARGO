mod support;

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use gateway::build_router;
use http_body_util::BodyExt;
use mongodb::{
    Client,
    bson::{Document, doc, oid::ObjectId},
};
use serde_json::{Value, json};
use tower::util::ServiceExt;

#[tokio::test]
async fn deve_persistir_task_e_retornar_201_quando_payload_for_valido() {
    support::ensure_mongo_service().await;
    let settings = support::mongo_settings("post_tasks_valid");
    support::prepare_tasks_collection(&settings)
        .await
        .expect("test setup should prepare the tasks collection");

    let app = build_router(gateway::GatewaySettings::new(settings.clone()));

    let response = app
        .oneshot(build_post_tasks_request(
            json!({ "title": "  Revisar recibos do mês  " }).to_string(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::CREATED);

    let payload = read_json_body(response).await;
    assert_eq!(payload["title"], "Revisar recibos do mês");
    assert_eq!(payload["status"], "pending");
    assert!(payload["created_at"].is_string());

    let id = payload["id"]
        .as_str()
        .expect("response should expose the Mongo-generated id");

    let persisted = load_persisted_task(&settings, id).await;

    assert_eq!(
        persisted.get_str("title").expect("title should exist"),
        "Revisar recibos do mês"
    );
    assert_eq!(
        persisted.get_str("status").expect("status should exist"),
        "pending"
    );
    assert!(persisted.get_datetime("created_at").is_ok());
}

#[tokio::test]
async fn deve_aceitar_post_tasks_sem_authorization_nesta_foundation() {
    support::ensure_mongo_service().await;
    let settings = support::mongo_settings("post_tasks_without_auth");
    support::prepare_tasks_collection(&settings)
        .await
        .expect("test setup should prepare the tasks collection");

    let app = build_router(gateway::GatewaySettings::new(settings));

    let response = app
        .oneshot(build_post_tasks_request(
            json!({ "title": "Criar task sem auth" }).to_string(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn deve_retornar_400_quando_json_for_malformado() {
    let app = build_router(support::dummy_settings());

    let response = app
        .oneshot(build_post_tasks_request(
            r#"{"title":"faltando fechamento""#.into(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let payload = read_json_body(response).await;
    assert_eq!(payload["error"]["code"], "BAD_REQUEST");
}

#[tokio::test]
async fn deve_retornar_400_quando_content_type_for_invalido() {
    let app = build_router(support::dummy_settings());

    let response = app
        .oneshot(build_post_tasks_request(
            json!({ "title": "Tipo inválido" }).to_string(),
            Some("text/plain"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let payload = read_json_body(response).await;
    assert_eq!(payload["error"]["code"], "BAD_REQUEST");
}

#[tokio::test]
async fn deve_retornar_422_quando_title_estiver_ausente() {
    assert_validation_failure(json!({})).await;
}

#[tokio::test]
async fn deve_retornar_422_quando_title_nao_for_string() {
    assert_validation_failure(json!({ "title": 123 })).await;
}

#[tokio::test]
async fn deve_retornar_422_quando_title_ultrapassar_120_caracteres() {
    assert_validation_failure(json!({ "title": "a".repeat(121) })).await;
}

#[tokio::test]
async fn deve_retornar_422_quando_payload_conter_campos_extras() {
    assert_validation_failure(json!({
        "title": "Task com extra",
        "description": "fora da foundation"
    }))
    .await;
}

#[tokio::test]
async fn deve_retornar_422_quando_title_for_vazio_ou_blank() {
    assert_validation_failure(json!({ "title": "   " })).await;
}

#[tokio::test]
async fn deve_retornar_503_quando_mongo_estiver_indisponivel() {
    let app = build_router(gateway::GatewaySettings::new(gateway::MongoSettings::new(
        "mongodb://admin:change_me@127.0.0.1:1/?authSource=admin&serverSelectionTimeoutMS=1000",
        "largo_mongo_unavailable",
    )));

    let response = app
        .oneshot(build_post_tasks_request(
            json!({ "title": "Mongo indisponível" }).to_string(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let payload = read_json_body(response).await;
    assert_eq!(payload["error"]["code"], "TASK_PERSISTENCE_FAILED");
}

#[tokio::test]
async fn deve_retornar_503_quando_banco_nao_estiver_preparado() {
    support::ensure_mongo_service().await;
    let settings = support::mongo_settings("post_tasks_unprepared_db");
    let app = build_router(gateway::GatewaySettings::new(settings));

    let response = app
        .oneshot(build_post_tasks_request(
            json!({ "title": "Banco não preparado" }).to_string(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let payload = read_json_body(response).await;
    assert_eq!(payload["error"]["code"], "TASK_PERSISTENCE_FAILED");
}

#[tokio::test]
async fn deve_retornar_503_quando_persistencia_falhar_no_mongo() {
    support::ensure_mongo_service().await;
    let settings = support::mongo_settings("post_tasks_persistence_failure");
    support::prepare_rejecting_tasks_collection(&settings)
        .await
        .expect("test setup should force persistence failure at Mongo layer");

    let app = build_router(gateway::GatewaySettings::new(settings));

    let response = app
        .oneshot(build_post_tasks_request(
            json!({ "title": "Falha ao persistir" }).to_string(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let payload = read_json_body(response).await;
    assert_eq!(payload["error"]["code"], "TASK_PERSISTENCE_FAILED");
}

async fn assert_validation_failure(body: Value) {
    let app = build_router(support::dummy_settings());

    let response = app
        .oneshot(build_post_tasks_request(
            body.to_string(),
            Some("application/json"),
            None,
        ))
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let payload = read_json_body(response).await;
    assert_eq!(payload["error"]["code"], "TASK_VALIDATION_FAILED");
}

fn build_post_tasks_request(
    body: String,
    content_type: Option<&str>,
    authorization: Option<&str>,
) -> Request<Body> {
    let mut builder = Request::builder().method("POST").uri("/tasks");

    if let Some(content_type) = content_type {
        builder = builder.header("content-type", content_type);
    }

    if let Some(authorization) = authorization {
        builder = builder.header("authorization", authorization);
    }

    builder
        .body(Body::from(body))
        .expect("request should be valid")
}

async fn read_json_body(response: Response<Body>) -> Value {
    serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should be readable")
            .to_bytes(),
    )
    .expect("response body should be valid JSON")
}

async fn load_persisted_task(settings: &gateway::MongoSettings, id: &str) -> Document {
    let client = Client::with_uri_str(&settings.uri)
        .await
        .expect("test should connect to Mongo");

    client
        .database(&settings.database_name)
        .collection::<Document>("tasks")
        .find_one(doc! { "_id": ObjectId::parse_str(id).expect("id should be a valid ObjectId") })
        .await
        .expect("query should succeed")
        .expect("task should have been persisted")
}
