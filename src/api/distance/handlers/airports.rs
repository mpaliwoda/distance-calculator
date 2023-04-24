#![allow(non_camel_case_types)]

use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnprocessableEntity};
use actix_web::web::Data;
use futures::future::join_all;
use itertools::Itertools;
use paperclip::actix::web::Json;
use paperclip::actix::{api_v2_operation, post};
use serde_json::json;
use serde_valid::Validate;
use tracing::log;

use crate::api::distance::schemas::{
    AirportCoordinates, AirportDistanceRequest, AirportDistanceResponse, AirportRoutePart,
};
use crate::models::{Airport, Coordinates};
use crate::services::app_state::AppState;
use crate::services::distance::{DistanceCalculator, DistanceCalculatorFactory};

#[api_v2_operation]
#[post("/calculate_distance/airports")]
pub async fn airports_handler(
    request: Json<AirportDistanceRequest>,
    app_state: Data<AppState>,
) -> Result<Json<AirportDistanceResponse>, actix_web::Error> {
    let request = request.into_inner();

    if let Err(validation_errors) = request.validate() {
        log::warn!("Failed to validate request: {validation_errors:?}");
        return Err(ErrorBadRequest(validation_errors));
    }

    let airports_results = join_all(
        request
            .route
            .iter()
            .map(|iata| app_state.airports_repository.fetch_airport_by_iata(iata)),
    )
    .await;

    let mut airports = vec![];
    let mut missing_airports = vec![];

    for (iata, airport_result) in request.route.iter().zip(airports_results.into_iter()) {
        match airport_result {
            Ok(airport) => match airport {
                Some(airport) => airports.push(airport),
                None => {
                    log::warn!("Missing airport: {iata}");
                    missing_airports.push(iata.as_str())
                }
            },
            Err(e) => {
                log::error!("Failed to fetch airport from database: {e}");
                return Err(ErrorInternalServerError(json!({"error": "Database fail"})));
            }
        };
    }

    if !missing_airports.is_empty() {
        return Err(ErrorUnprocessableEntity(json!({
            "error": "Some airports are missing in our database",
            "details": {
                "missing_airports": missing_airports
            }
        })));
    }

    let calculator = DistanceCalculatorFactory::create(&request.formula, &request.datum);
    let distances = match calculate_distances(&airports, calculator.as_ref()) {
        Ok(distances) => distances,
        Err(e) => return Err(e),
    };

    let total_distance = distances.iter().map(|part| part.distance).sum();

    Ok(Json(AirportDistanceResponse {
        distances,
        total_distance,
        datum: request.datum,
        formula: request.formula,
    }))
}

fn calculate_distances(
    route: &[Airport],
    calculator: &dyn DistanceCalculator,
) -> Result<Vec<AirportRoutePart>, actix_web::Error> {
    let mut distances: Vec<AirportRoutePart> = Vec::new();

    for (from, to) in route.iter().tuple_windows() {
        let from_coords = Coordinates::new(from.lat_decimal, from.lon_decimal);
        let to_coords = Coordinates::new(to.lat_decimal, to.lon_decimal);

        let distance = match calculator.calculate_distance(&from_coords, &to_coords) {
            Ok(distance) => distance,
            Err(e) => {
                log::warn!("failed to calculate distance: {e}, from: {from:?}, to: {to:?}");
                return Err(ErrorUnprocessableEntity(e));
            }
        };

        distances.push(AirportRoutePart {
            from: AirportCoordinates {
                iata_code: from.iata_code.to_owned(),
                coordinates: from_coords,
            },
            to: AirportCoordinates {
                iata_code: to.iata_code.to_owned(),
                coordinates: to_coords,
            },
            distance,
        })
    }

    Ok(distances)
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use actix_web::http::StatusCode;
    use sqlx::SqlitePool;

    use super::*;
    use crate::{
        models::{Datum, Formula},
        services::{
            airports::GlobalAirportsRepository,
            healthcheck::{
                hostname_provider::MockSuccessfulHostnameProvider, time_provider::MockTimeProvider,
            },
        },
        DATABASE_URL,
    };

    async fn app_state() -> Data<AppState> {
        let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();
        let state = AppState::new(
            Box::new(MockSuccessfulHostnameProvider::new("test".into())),
            Box::new(MockTimeProvider::new(SystemTime::now())),
            Box::new(GlobalAirportsRepository::new(pool).await),
        );

        Data::new(state)
    }

    #[actix_web::test]
    async fn test_airports_handler() {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(app_state().await)
                .service(airports_handler),
        )
        .await;

        let request = AirportDistanceRequest {
            route: vec!["LHR".to_owned(), "JFK".to_owned()],
            formula: Formula::Haversine,
            datum: Datum::WGS84,
        };

        let request = actix_web::test::TestRequest::post()
            .uri("/calculate_distance/airports")
            .set_json(&request);

        let response = actix_web::test::call_service(&app, request.to_request()).await;

        println!("{:?}", response.response().body());
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_airports_handler_invalid_request() {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(app_state().await)
                .service(airports_handler),
        )
        .await;

        let request = AirportDistanceRequest {
            route: vec!["LHR".to_owned()],
            formula: Formula::Haversine,
            datum: Datum::WGS84,
        };

        let request = actix_web::test::TestRequest::post()
            .uri("/calculate_distance/airports")
            .set_json(&request);

        let response = actix_web::test::call_service(&app, request.to_request()).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_airport_handler_no_airport_found() {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(app_state().await)
                .service(airports_handler),
        )
        .await;

        let request = AirportDistanceRequest {
            route: vec!["LHR".to_owned(), "XXX".to_owned()],
            formula: Formula::Haversine,
            datum: Datum::WGS84,
        };

        let request = actix_web::test::TestRequest::post()
            .uri("/calculate_distance/airports")
            .set_json(&request);

        let response = actix_web::test::call_service(&app, request.to_request()).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
