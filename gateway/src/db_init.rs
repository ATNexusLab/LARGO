use anyhow::{Context, Result};
use mongodb::{
    Client,
    bson::doc,
    error::{Error, ErrorKind},
};

use crate::MongoSettings;
use crate::tasks::{TASKS_COLLECTION_NAME, tasks_collection_validator};

pub async fn run_db_init(settings: &MongoSettings) -> Result<()> {
    let client = Client::with_uri_str(&settings.uri)
        .await
        .context("failed to create MongoDB client for db-init")?;
    let database = client.database(&settings.database_name);

    ensure_tasks_collection(&database).await?;
    drop_non_foundation_indexes(&database).await?;

    Ok(())
}

async fn ensure_tasks_collection(database: &mongodb::Database) -> Result<()> {
    let validator = tasks_collection_validator();

    match database
        .run_command(doc! {
            "create": TASKS_COLLECTION_NAME,
            "validator": validator.clone(),
            "validationLevel": "strict",
            "validationAction": "error",
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(error) if is_namespace_exists_error(&error) => {
            database
                .run_command(doc! {
                    "collMod": TASKS_COLLECTION_NAME,
                    "validator": validator,
                    "validationLevel": "strict",
                    "validationAction": "error",
                })
                .await
                .context("failed to update tasks collection validator during db-init")?;

            Ok(())
        }
        Err(error) => {
            Err(error).context("failed to create tasks collection with foundation validator")
        }
    }
}

async fn drop_non_foundation_indexes(database: &mongodb::Database) -> Result<()> {
    let collection = database.collection::<mongodb::bson::Document>(TASKS_COLLECTION_NAME);
    let index_names = collection
        .list_index_names()
        .await
        .context("failed to list tasks indexes during db-init")?;

    for index_name in index_names {
        if index_name != "_id_" {
            collection
                .drop_index(&index_name)
                .await
                .with_context(|| format!("failed to drop unexpected tasks index `{index_name}`"))?;
        }
    }

    Ok(())
}

fn is_namespace_exists_error(error: &Error) -> bool {
    matches!(
        error.kind.as_ref(),
        ErrorKind::Command(command_error) if command_error.code == 48
    )
}
