use anyhow::{Context, Result};
use mongodb::{Client, bson::doc};

use crate::MongoSettings;

pub async fn check_mongo_connectivity(settings: &MongoSettings) -> Result<()> {
    let client = Client::with_uri_str(&settings.uri)
        .await
        .context("failed to create MongoDB client for connectivity validation")?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await
        .context("failed to validate MongoDB connectivity with an explicit ping")?;

    Ok(())
}
