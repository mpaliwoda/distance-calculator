use super::{DistanceCalculationError, DistanceCalculator};
use crate::models::Coordinates;

pub(super) struct HaversineDistanceCalculator {
    earth_radius_in_kilometers: f64,
}

impl HaversineDistanceCalculator {
    pub fn new(earth_radius_in_kilometers: f64) -> Self {
        Self {
            earth_radius_in_kilometers,
        }
    }
}

impl DistanceCalculator for HaversineDistanceCalculator {
    fn calculate_distance(
        &self,
        from: &Coordinates,
        to: &Coordinates,
    ) -> Result<f64, DistanceCalculationError> {
        let lat1 = from.latitude.to_radians();
        let lon1 = from.longitude.to_radians();

        let lat2 = to.latitude.to_radians();
        let lon2 = to.longitude.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();

        Ok(self.earth_radius_in_kilometers * c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let calculator = HaversineDistanceCalculator::new(6371.0);

        let from = Coordinates {
            latitude: 52.2296756,
            longitude: 21.0122287,
        };
        let to = Coordinates {
            latitude: 52.406374,
            longitude: 16.9251681,
        };

        let distance = calculator
            .calculate_distance(&from, &to)
            .expect("Should not fail");

        assert!((distance - 278.4581).abs() < 0.0001);
    }
}
