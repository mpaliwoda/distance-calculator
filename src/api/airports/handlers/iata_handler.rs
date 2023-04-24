#![allow(non_camel_case_types)]

use crate::api::airports::schemas::UniqueIatasResponse;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Data;
use paperclip::actix::web::Json;
use paperclip::actix::{api_v2_operation, get};
use serde_json::json;

use crate::services::app_state::AppState;

#[api_v2_operation]
#[get("/airports/iatas")]
pub async fn unique_iatas_handler(
    data: Data<AppState>,
) -> Result<Json<UniqueIatasResponse>, actix_web::Error> {
    let iatas = data.airports_repository.unique_airport_iatas().await;

    match iatas {
        Ok(iatas) => Ok(Json(UniqueIatasResponse { iatas })),
        Err(_) => Err(ErrorInternalServerError(json!({"error": "Database fail"}))),
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use actix_web::{http, test, App};
    use sqlx::SqlitePool;

    use crate::{
        services::{
            airports::GlobalAirportsRepository,
            app_state::AppState,
            healthcheck::{
                hostname_provider::MockSuccessfulHostnameProvider, time_provider::MockTimeProvider,
            },
        },
        DATABASE_URL,
    };

    use super::*;

    #[actix_web::test]
    async fn test_unique_iatas_handler() {
        let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
        let state = AppState::new(
            Box::new(MockSuccessfulHostnameProvider::new("test".into())),
            Box::new(MockTimeProvider::new(SystemTime::now())),
            Box::new(GlobalAirportsRepository::new(pool).await),
        );

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(state))
                .service(unique_iatas_handler),
        )
        .await;

        let req = test::TestRequest::get().uri("/airports/iatas").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body: UniqueIatasResponse = test::read_body_json(resp).await;
        assert!(body.iatas.len() > 0);
    }
}
