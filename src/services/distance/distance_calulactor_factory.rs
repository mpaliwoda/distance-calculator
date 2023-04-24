use crate::models::earth::{nad27, nad83, wgs84};
use crate::models::{Datum, Formula};

use super::great_circle::GreatCircleDistanceCalculator;
use super::haversine::HaversineDistanceCalculator;
use super::vincenty::VincentyDistanceCalculator;
use super::DistanceCalculator;

pub struct DistanceCalculatorFactory;

impl DistanceCalculatorFactory {
    pub fn create(formula: &Formula, datum: &Datum) -> Box<dyn DistanceCalculator> {
        let earth_radius_in_kilometers;
        let inverse_flattening_factor;
        let semi_minor_axis_in_kilometers;

        match datum {
            Datum::WGS84 => {
                earth_radius_in_kilometers = wgs84::EARTH_RADIUS_IN_KILOMETERS;
                inverse_flattening_factor = wgs84::INVERSE_FLATTENING_FACTOR;
                semi_minor_axis_in_kilometers = wgs84::SEMI_MINOR_AXIS_IN_KILOMETERS;
            }
            Datum::NAD27 => {
                earth_radius_in_kilometers = nad27::EARTH_RADIUS_IN_KILOMETERS;
                inverse_flattening_factor = nad27::INVERSE_FLATTENING_FACTOR;
                semi_minor_axis_in_kilometers = nad27::SEMI_MINOR_AXIS_IN_KILOMETERS;
            }
            Datum::NAD83 => {
                earth_radius_in_kilometers = nad83::EARTH_RADIUS_IN_KILOMETERS;
                inverse_flattening_factor = nad83::INVERSE_FLATTENING_FACTOR;
                semi_minor_axis_in_kilometers = nad83::SEMI_MINOR_AXIS_IN_KILOMETERS;
            }
        }

        match formula {
            Formula::GreatCircle => Box::new(GreatCircleDistanceCalculator::new(
                earth_radius_in_kilometers,
            )),
            Formula::Haversine => {
                Box::new(HaversineDistanceCalculator::new(earth_radius_in_kilometers))
            }
            Formula::Vincenty => Box::new(VincentyDistanceCalculator::new(
                earth_radius_in_kilometers,
                semi_minor_axis_in_kilometers,
                inverse_flattening_factor,
            )),
        }
    }
}
