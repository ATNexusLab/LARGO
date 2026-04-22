use std::process::ExitCode;

use gateway::{MongoSettings, run_db_init};

#[tokio::main]
async fn main() -> ExitCode {
    let settings = match MongoSettings::from_env() {
        Ok(settings) => settings,
        Err(error) => {
            eprintln!("db-init configuration error: {error}");
            return ExitCode::FAILURE;
        }
    };

    match run_db_init(&settings).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("db-init execution error: {error}");
            ExitCode::FAILURE
        }
    }
}
