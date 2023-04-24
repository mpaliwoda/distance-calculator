pub(crate) mod great_circle;

mod distance_calulactor_factory;
mod haversine;
mod vincenty;
use std::error::Error;

pub use distance_calulactor_factory::DistanceCalculatorFactory;

use crate::models::Coordinates;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistanceCalculationError(pub String);

impl Error for DistanceCalculationError {}
impl std::fmt::Display for DistanceCalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DistanceCalculationError: {}", self.0)
    }
}

pub trait DistanceCalculator {
    fn calculate_distance(
        &self,
        from: &Coordinates,
        to: &Coordinates,
    ) -> Result<f64, DistanceCalculationError>;
}
