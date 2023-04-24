use super::{DistanceCalculationError, DistanceCalculator};
use crate::models::Coordinates;

pub(super) struct GreatCircleDistanceCalculator {
    earth_radius_in_kilometers: f64,
}

impl GreatCircleDistanceCalculator {
    pub fn new(earth_radius_in_kilometers: f64) -> Self {
        GreatCircleDistanceCalculator {
            earth_radius_in_kilometers,
        }
    }
}

impl DistanceCalculator for GreatCircleDistanceCalculator {
    fn calculate_distance(
        &self,
        from: &Coordinates,
        to: &Coordinates,
    ) -> Result<f64, DistanceCalculationError> {
        let lat1 = from.latitude.to_radians();
        let lon1 = from.longitude.to_radians();

        let lat2 = to.latitude.to_radians();
        let lon2 = to.longitude.to_radians();

        let d_lon = (lon2 - lon1).abs();

        let central_angle =
            (lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * d_lon.cos()).acos();

        Ok(self.earth_radius_in_kilometers * central_angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let calculator = GreatCircleDistanceCalculator::new(6378.1);
        let expected_distance = 246.8;

        let krk = Coordinates::new(50.0770, 19.7881);
        let waw = Coordinates::new(52.1672, 20.9679);

        let distance = calculator
            .calculate_distance(&krk, &waw)
            .expect("Should not fail");

        assert!((expected_distance - distance).abs() < 0.1);
    }
}
