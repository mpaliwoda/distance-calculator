use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::models::{Coordinates, Datum, Formula};

#[derive(Debug, Deserialize, Serialize, Validate, Apiv2Schema)]
pub struct AirportDistanceRequest {
    #[validate(min_items = 2)]
    pub route: Vec<String>,
    #[serde(default)]
    pub formula: Formula,
    #[serde(default)]
    pub datum: Datum,
}

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct AirportCoordinates {
    pub iata_code: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct AirportRoutePart {
    pub from: AirportCoordinates,
    pub to: AirportCoordinates,
    pub distance: f64,
}

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct AirportDistanceResponse {
    pub distances: Vec<AirportRoutePart>,
    pub formula: Formula,
    pub datum: Datum,
    pub total_distance: f64,
}
