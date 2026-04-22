#![allow(dead_code)]

use std::{
    path::PathBuf,
    process::Command,
    sync::OnceLock,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Result, anyhow, bail};
use gateway::{GatewaySettings, MongoSettings};
use mongodb::{Client, bson::doc};

static MONGO_BOOTSTRAP: OnceLock<()> = OnceLock::new();

pub async fn ensure_mongo_service() {
    MONGO_BOOTSTRAP.get_or_init(|| {
        bootstrap_mongo_service().expect("test MongoDB service should bootstrap successfully");
    });

    wait_for_mongo()
        .await
        .expect("test MongoDB service should become reachable");
}

pub fn dummy_settings() -> GatewaySettings {
    GatewaySettings::new(MongoSettings::new(
        "mongodb://placeholder:placeholder@localhost:27017/?authSource=admin",
        "largo_placeholder",
    ))
}

pub fn mongo_settings(test_name: &str) -> MongoSettings {
    let username = mongo_username();
    let password = mongo_password();
    let port = mongo_port();
    let database = std::env::var("MONGO_DB_NAME").unwrap_or_else(|_| "largo".into());
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX_EPOCH")
        .as_nanos();

    MongoSettings::new(
        format!(
            "mongodb://{username}:{password}@127.0.0.1:{port}/?authSource=admin&serverSelectionTimeoutMS=1000"
        ),
        format!("{database}_{test_name}_{suffix}"),
    )
}

pub async fn prepare_tasks_collection(settings: &MongoSettings) -> Result<()> {
    apply_tasks_validator(settings, tasks_validator()).await
}

pub async fn prepare_rejecting_tasks_collection(settings: &MongoSettings) -> Result<()> {
    apply_tasks_validator(settings, rejecting_tasks_validator()).await
}

fn bootstrap_mongo_service() -> Result<()> {
    let output = Command::new("docker")
        .args(["compose", "up", "-d", "mongo"])
        .current_dir(repo_root())
        .env("MONGO_INITDB_ROOT_USERNAME", mongo_username())
        .env("MONGO_INITDB_ROOT_PASSWORD", mongo_password())
        .env(
            "MONGO_DB_NAME",
            std::env::var("MONGO_DB_NAME").unwrap_or_else(|_| "largo".into()),
        )
        .output()?;

    if output.status.success() {
        return Ok(());
    }

    bail!(
        "docker compose up -d mongo failed: {}",
        String::from_utf8_lossy(&output.stderr)
    )
}

async fn wait_for_mongo() -> Result<()> {
    let settings = mongo_settings("bootstrap_ping");

    for _ in 0..30 {
        let client = Client::with_uri_str(&settings.uri).await?;
        let ping = client
            .database("admin")
            .run_command(doc! { "ping": 1 })
            .await;

        if ping.is_ok() {
            return Ok(());
        }

        thread::sleep(Duration::from_secs(1));
    }

    Err(anyhow!(
        "MongoDB did not answer ping within the expected timeout"
    ))
}

async fn apply_tasks_validator(
    settings: &MongoSettings,
    validator: mongodb::bson::Document,
) -> Result<()> {
    let client = Client::with_uri_str(&settings.uri).await?;
    let database = client.database(&settings.database_name);

    match database
        .run_command(doc! {
            "create": "tasks",
            "validator": validator.clone(),
            "validationLevel": "strict",
            "validationAction": "error",
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(error) if error.to_string().contains("NamespaceExists") => {
            database
                .run_command(doc! {
                    "collMod": "tasks",
                    "validator": validator,
                    "validationLevel": "strict",
                    "validationAction": "error",
                })
                .await?;
            Ok(())
        }
        Err(error) => Err(error.into()),
    }
}

fn tasks_validator() -> mongodb::bson::Document {
    doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["title", "status", "created_at"],
            "properties": {
                "title": {
                    "bsonType": "string",
                    "minLength": 1,
                    "maxLength": 120,
                },
                "status": {
                    "enum": ["pending", "in_progress", "done", "blocked"],
                },
                "created_at": {
                    "bsonType": "date",
                },
            },
        },
    }
}

fn rejecting_tasks_validator() -> mongodb::bson::Document {
    doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["title", "status", "created_at"],
            "properties": {
                "title": {
                    "bsonType": "string",
                    "minLength": 121,
                },
                "status": {
                    "enum": ["pending"],
                },
                "created_at": {
                    "bsonType": "date",
                },
            },
        },
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("gateway crate should live inside the repository root")
        .to_path_buf()
}

fn mongo_username() -> String {
    std::env::var("MONGO_INITDB_ROOT_USERNAME").unwrap_or_else(|_| "admin".into())
}

fn mongo_password() -> String {
    std::env::var("MONGO_INITDB_ROOT_PASSWORD").unwrap_or_else(|_| "change_me".into())
}

fn mongo_port() -> String {
    std::env::var("MONGO_PORT").unwrap_or_else(|_| "27017".into())
}
