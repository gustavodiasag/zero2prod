use axum_test::TestServer;
use zero2prod::startup::app;

#[cfg(test)]
async fn test_app() -> TestServer {
    let test_app = app();

    TestServer::new(test_app).unwrap()
}

#[cfg(test)]
mod health_check_endpoint {
    use crate::test_app;

    #[tokio::test]
    async fn health_check_works() {
        let server = test_app().await;

        let response = server.get("/health_check").await;

        assert!(response.status_code().is_success());
        response.assert_text("");
    }
}

#[cfg(test)]
mod subscription_endpoint {
    use serde::Serialize;

    use crate::test_app;

    #[derive(Serialize)]
    struct Subscription {
        name: String,
        email: String,
    }

    #[tokio::test]
    async fn test_valid_subscription() {
        let server = test_app().await;

        let body = Subscription {
            name: "foo".to_string(),
            email: "bar".to_string(),
        };
        let response = server
            .post("/subscriptions")
            .form::<Subscription>(&body)
            .await;

        assert!(response.status_code().is_success());
    }

    #[derive(Serialize)]
    struct Name {
        name: String,
    }

    #[tokio::test]
    async fn test_invalid_subscription() {
        let server = test_app().await;

        let body = Name {
            name: "foo".to_string(),
        };
        let response = server
            .post("/subscriptions")
            .expect_failure()
            .form::<Name>(&body)
            .await;

        assert!(response.status_code().is_client_error());
    }
}
