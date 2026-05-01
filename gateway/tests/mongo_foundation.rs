mod support;

use gateway::check_mongo_connectivity;

#[tokio::test]
async fn deve_validar_conectividade_mongo_na_foundation() {
    support::ensure_mongo_service().await;
    let settings = support::mongo_settings("connectivity");

    check_mongo_connectivity(&settings)
        .await
        .expect("Mongo connectivity should be validated explicitly during foundation bootstrap");
}
