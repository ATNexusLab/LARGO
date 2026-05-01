mod support;

use futures_util::TryStreamExt;
use gateway::run_db_init;
use mongodb::{
    Client,
    bson::{DateTime, doc},
};

#[tokio::test]
async fn deve_executar_smoke_path_idempotente_do_db_init_para_tasks() {
    support::ensure_mongo_service().await;
    let settings = support::mongo_settings("db_init_smoke");

    run_db_init(&settings)
        .await
        .expect("first execution should prepare an empty database");
    run_db_init(&settings)
        .await
        .expect("second execution should be idempotent");

    let client = Client::with_uri_str(&settings.uri)
        .await
        .expect("test should connect to Mongo");
    let database = client.database(&settings.database_name);

    let collections = database
        .list_collection_names()
        .await
        .expect("listing collections should succeed");
    assert!(collections.iter().any(|name| name == "tasks"));

    let indexes = database
        .collection::<mongodb::bson::Document>("tasks")
        .list_indexes()
        .await
        .expect("listing indexes should succeed")
        .try_collect::<Vec<_>>()
        .await
        .expect("index cursor should be consumable");
    assert_eq!(
        indexes.len(),
        1,
        "foundation should rely only on the default _id index"
    );

    database
        .collection::<mongodb::bson::Document>("tasks")
        .insert_one(doc! {
            "title": "Smoke path",
            "status": "pending",
            "created_at": DateTime::now(),
        })
        .await
        .expect("db-init should leave the collection accepting valid documents");

    let invalid_insert = database
        .collection::<mongodb::bson::Document>("tasks")
        .insert_one(doc! {
            "status": "pending",
            "created_at": DateTime::now(),
        })
        .await;

    assert!(
        invalid_insert.is_err(),
        "db-init should apply a validator that rejects documents missing title"
    );
}
