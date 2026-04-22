mod support;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use gateway::build_router;
use tower::util::ServiceExt;

#[tokio::test]
async fn deve_responder_sucesso_no_healthcheck_do_gateway() {
    let app = build_router(support::dummy_settings());

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/healthz")
                .body(Body::empty())
                .expect("healthcheck request should be valid"),
        )
        .await
        .expect("router should answer the request");

    assert_eq!(response.status(), StatusCode::OK);
}
