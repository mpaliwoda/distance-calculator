use std::time::UNIX_EPOCH;

use actix_web::{get, web::Data, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::log::error;

use crate::services::app_state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub healthy: bool,
    pub timestamp: u64,
    pub hostname: String,
    pub message: String,
}

#[get("/health")]
pub async fn check_health(app_state: Data<AppState>) -> impl Responder {
    // we're going to store our error messages in a vector to provide better `unhealthy` message
    let mut errors = vec![];

    let timestamp = match app_state.time_provider.now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(e) => {
            error!("Failed to get current timestamp: {}", e);
            errors.push(e.to_string());
            0
        }
    };

    let hostname = match app_state.hostname_provider.get() {
        // hostname.get() returns os native string so we need to convert it to Rust string,
        // accepting the possible loss of data which for our use case should be good enough
        Ok(hostname) => hostname.to_string_lossy().to_string(),
        Err(e) => {
            error!("Failed to get hostname: {}", e);
            errors.push(e.to_string());
            "unknown".to_string()
        }
    };

    match errors.len() {
        0 => HttpResponse::Ok().json(HealthResponse {
            healthy: true,
            timestamp,
            hostname,
            message: "ok".to_string(),
        }),
        _ => HttpResponse::InternalServerError().json(HealthResponse {
            healthy: false,
            timestamp,
            hostname,
            message: errors.join(";"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::services::airports::DummyAirportsRepository;
    use crate::services::healthcheck::hostname_provider::MockFailedHostnameProvider;
    use crate::services::healthcheck::{
        hostname_provider::MockSuccessfulHostnameProvider, time_provider::MockTimeProvider,
    };

    use super::*;
    use actix_web::{test, App};

    async fn test_health_check(
        app_state: AppState,
        expected_response: HealthResponse,
        expected_status_code: u16,
    ) {
        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(app_state))
                .service(check_health),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), expected_status_code);

        let resp: HealthResponse = test::read_body_json(resp).await;
        assert_eq!(resp.healthy, expected_response.healthy);
        assert_eq!(resp.message, expected_response.message);
        assert_eq!(resp.timestamp, expected_response.timestamp);
        assert_eq!(resp.hostname, expected_response.hostname);
    }

    #[actix_web::test]
    async fn test_successful_health_check() {
        let mock_time_provider = MockTimeProvider::new(UNIX_EPOCH + Duration::from_secs(123456789));
        let mock_hostname_provider = MockSuccessfulHostnameProvider::new("test-hostname".into());
        let app_state = AppState::new(
            Box::new(mock_hostname_provider),
            Box::new(mock_time_provider),
            Box::new(DummyAirportsRepository::new(vec![])),
        );

        let expected = HealthResponse {
            healthy: true,
            timestamp: 123456789,
            hostname: "test-hostname".to_string(),
            message: "ok".to_string(),
        };
        let expected_status_code = 200;

        test_health_check(app_state, expected, expected_status_code).await;
    }

    #[actix_web::test]
    async fn test_health_check_time_unsuccessful() {
        let mock_time_provider = MockTimeProvider::new(UNIX_EPOCH - Duration::from_secs(1));
        let mock_hostname_provider = MockSuccessfulHostnameProvider::new("test-hostname".into());
        let app_state = AppState::new(
            Box::new(mock_hostname_provider),
            Box::new(mock_time_provider),
            Box::new(DummyAirportsRepository::new(vec![])),
        );

        let expected = HealthResponse {
            healthy: false,
            timestamp: 0,
            hostname: "test-hostname".to_string(),
            message: "second time provided was later than self".to_string(),
        };

        test_health_check(app_state, expected, 500).await;
    }

    #[actix_web::test]
    async fn test_health_check_hostname_unsuccessful() {
        let mock_time_provider = MockTimeProvider::new(UNIX_EPOCH + Duration::from_secs(123456789));
        let mock_hostname_provider = MockFailedHostnameProvider::new();
        let app_state = AppState::new(
            Box::new(mock_hostname_provider),
            Box::new(mock_time_provider),
            Box::new(DummyAirportsRepository::new(vec![])),
        );

        let expected = HealthResponse {
            healthy: false,
            timestamp: 123456789,
            hostname: "unknown".to_string(),
            message: "mock error".to_string(),
        };
        let expected_status_code = 500;

        test_health_check(app_state, expected, expected_status_code).await;
    }

    #[actix_web::test]
    async fn test_health_check_time_and_hostname_unsuccessful() {
        let mock_time_provider = MockTimeProvider::new(UNIX_EPOCH - Duration::from_secs(1));
        let mock_hostname_provider = MockFailedHostnameProvider::new();
        let app_state = AppState::new(
            Box::new(mock_hostname_provider),
            Box::new(mock_time_provider),
            Box::new(DummyAirportsRepository::new(vec![])),
        );

        let expected = HealthResponse {
            healthy: false,
            timestamp: 0,
            hostname: "unknown".to_string(),
            message: "second time provided was later than self;mock error".into(),
        };
        let expected_status_code = 500;

        test_health_check(app_state, expected, expected_status_code).await;
    }
}
