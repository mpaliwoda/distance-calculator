#![allow(non_camel_case_types)]

use actix_web::error::{ErrorBadRequest, ErrorUnprocessableEntity};
use itertools::Itertools;
use paperclip::actix::web::Json;
use paperclip::actix::{api_v2_operation, post};
use serde_valid::Validate;
use tracing::log;

use crate::api::distance::schemas::{
    CoordinatesDistanceRequest, CoordinatesDistanceResponse, CoordinatesRoutePart,
};
use crate::services::distance::DistanceCalculatorFactory;

#[api_v2_operation]
#[post("/calculate_distance/coordinates")]
pub async fn coordinates_handler(
    request: Json<CoordinatesDistanceRequest>,
) -> Result<Json<CoordinatesDistanceResponse>, actix_web::Error> {
    let request = request.into_inner();

    if let Err(validation_errors) = request.validate() {
        log::warn!("Failed to validate request: {validation_errors:?}");
        return Err(ErrorBadRequest(validation_errors));
    }

    let calculator = DistanceCalculatorFactory::create(&request.formula, &request.datum);

    let mut distances: Vec<CoordinatesRoutePart> = Vec::new();
    let mut total_distance = 0.0;

    for (from, to) in request.route.iter().tuple_windows() {
        match calculator.calculate_distance(from, to) {
            Ok(distance) => {
                distances.push(CoordinatesRoutePart {
                    from: from.clone(),
                    to: to.clone(),
                    distance,
                });
                total_distance += distance;
            }
            Err(e) => {
                log::warn!("failed to calculate distance: {e}, from: {from:?}, to: {to:?}");
                return Err(ErrorUnprocessableEntity(e));
            }
        }
    }

    Ok(Json(CoordinatesDistanceResponse {
        distances,
        total_distance,
        datum: request.datum,
        formula: request.formula,
    }))
}

#[cfg(test)]
mod tests {
    use actix_web::{http, test, App};

    use crate::models::{Coordinates, Datum, Formula};

    use super::*;

    #[actix_web::test]
    async fn test_distance_handler_full_request() {
        let mut app = test::init_service(App::new().service(coordinates_handler)).await;

        let req = test::TestRequest::post()
            .uri("/calculate_distance/coordinates")
            .set_json(&CoordinatesDistanceRequest {
                route: vec![
                    Coordinates::new(0.0, 0.0),
                    Coordinates::new(0.0, 1.0),
                    Coordinates::new(1.0, 1.0),
                    Coordinates::new(2.0, 4.0),
                ],
                formula: Formula::GreatCircle,
                datum: Datum::WGS84,
            })
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response: CoordinatesDistanceResponse = serde_json::from_slice(&body).unwrap();

        assert!((response.distances[0].distance - 111.3194907).abs() < 0.000001);
        assert!((response.distances[1].distance - 111.3194907).abs() < 0.000001);
        assert!((response.distances[2].distance - 351.9105211).abs() < 0.000001);
        assert!((response.total_distance - 574.5495025).abs() < 0.000001);
    }

    #[actix_web::test]
    async fn test_distance_handler_defaults() {
        let mut app = test::init_service(App::new().service(coordinates_handler)).await;

        let req = test::TestRequest::post()
            .uri("/calculate_distance/coordinates")
            .set_payload(
                r#"{
                "route": [
                    {
                        "latitude": 0.0,
                        "longitude": 0.0
                    },
                    {
                        "latitude": 0.0,
                        "longitude": 1.0
                    }
                ]
            }"#,
            )
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response: CoordinatesDistanceResponse = serde_json::from_slice(&body).unwrap();

        assert!((response.distances[0].distance - 111.3194907).abs() < 0.000001);
    }

    #[actix_web::test]
    async fn test_distance_handler_incorrect_request_body() {
        let mut app = test::init_service(App::new().service(coordinates_handler)).await;

        let req = test::TestRequest::post()
            .uri("/calculate_distance/coordinates")
            .set_payload(
                r#"{
                "route": [
                    {
                        "latitude": 0.0,
                        "longitude": 0.0
                    } 
                ]
            }"#,
            )
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_distance_handler_no_content_type() {
        let mut app = test::init_service(App::new().service(coordinates_handler)).await;

        let req = test::TestRequest::post()
            .uri("/calculate_distance/coordinates")
            .set_payload(
                r#"{
                "route": [
                    {
                        "latitude": 0.0,
                        "longitude": 0.0
                    } 
                ]
            }"#,
            )
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_distance_handler_vincenty() {
        let mut app = test::init_service(App::new().service(coordinates_handler)).await;

        let req = test::TestRequest::post()
            .uri("/calculate_distance/coordinates")
            .set_payload(
                r#"{
                "route": [
                    {
                        "latitude": 0.0,
                        "longitude": 0.0
                    },
                    {
                        "latitude": 0.0,
                        "longitude": 1.0
                    },
                    {
                        "latitude": 1.0,
                        "longitude": 1.0
                    },
                    {
                        "latitude": 2.0,
                        "longitude": 4.0
                    }
                ],
                "formula": "vincenty"
            }"#,
            )
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let response: CoordinatesDistanceResponse = serde_json::from_slice(&body).unwrap();

        assert!((response.distances[0].distance - 111.3194907).abs() < 0.000001);
        assert!((response.distances[1].distance - 110.5743885).abs() < 0.000001);
        assert!((response.distances[2].distance - 351.6765004).abs() < 0.000001);
    }

    #[actix_web::test]
    async fn test_handles_distance_calculation_error() {
        let mut app = test::init_service(App::new().service(coordinates_handler)).await;

        let req = test::TestRequest::post()
            .uri("/calculate_distance/coordinates")
            .set_payload(
                r#"{
                "route": [
                    {
                        "latitude": 0.0,
                        "longitude": 0.0
                    },
                    {
                        "latitude": 0.5,
                        "longitude": 179.7
                    }
                ],
                "formula": "vincenty",
                "datum": "wgs84"
            }"#,
            )
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }
}
