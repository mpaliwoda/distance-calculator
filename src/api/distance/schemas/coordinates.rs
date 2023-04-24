use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::models::{Coordinates, Datum, Formula};

#[derive(Debug, Deserialize, Serialize, Validate, Apiv2Schema)]
pub struct CoordinatesDistanceRequest {
    #[validate(min_items = 2)]
    pub route: Vec<Coordinates>,
    #[serde(default)]
    pub formula: Formula,
    #[serde(default)]
    pub datum: Datum,
}

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct CoordinatesRoutePart {
    pub from: Coordinates,
    pub to: Coordinates,
    pub distance: f64,
}

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct CoordinatesDistanceResponse {
    pub distances: Vec<CoordinatesRoutePart>,
    pub formula: Formula,
    pub datum: Datum,
    pub total_distance: f64,
}
