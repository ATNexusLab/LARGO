use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct GatewaySettings {
    pub mongo: MongoSettings,
}

impl GatewaySettings {
    pub fn new(mongo: MongoSettings) -> Self {
        Self { mongo }
    }
}

#[derive(Clone, Debug)]
pub struct MongoSettings {
    pub uri: String,
    pub database_name: String,
}

impl MongoSettings {
    pub fn new(uri: impl Into<String>, database_name: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            database_name: database_name.into(),
        }
    }

    pub fn from_env() -> Result<Self> {
        let uri = std::env::var("MONGO_URI").context("MONGO_URI must be set")?;
        let database_name = std::env::var("MONGO_DB_NAME").context("MONGO_DB_NAME must be set")?;

        Ok(Self::new(uri, database_name))
    }
}
