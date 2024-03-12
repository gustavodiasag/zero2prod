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

#[cfg(test)]
mod subscription_endpoint {
    use serde::Serialize;
    use axum::http::StatusCode;

    use crate::new_test_app;

    #[derive(Serialize)]
    struct SubscriberForm {
        name: String,
        email: String,
    }

    #[tokio::test]
    async fn valid_subscription() {
         let server = new_test_app().await;

         let body = SubscriberForm {
             name: "foo".to_string(),
             email: "bar".to_string(),
         };
         let response = server
            .post("/subscriptions")
            .form::<SubscriberForm>(&body)
            .await;

        assert!(response.status_code().is_success());
    }

    #[tokio::test]
    async fn invalid_subscription() {
        let server = new_test_app().await;

        let body = SubscriberForm {
            name: "".to_string(),
            email: "".to_string(),
        };
        let response = server
            .post("/subscriptions")
            .expect_failure()
            .form::<SubscriberForm>(&body)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }
}
