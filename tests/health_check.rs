use axum_test::TestServer;
use zero2prod::new_app;

#[cfg(test)]
async fn new_test_app() -> TestServer {
    let test_app = new_app();

    TestServer::new(test_app).unwrap()
}

#[cfg(test)]
mod health_check_endpoint {
    use crate::new_test_app;

    #[tokio::test]
    async fn health_check_works() {
        let server = new_test_app().await;

        let response = server.get("/health_check").await;

        assert!(response.status_code().is_success());
        response.assert_text("");
    }
}
