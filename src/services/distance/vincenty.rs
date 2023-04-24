use super::{DistanceCalculationError, DistanceCalculator};
use crate::models::Coordinates;

const MAX_ITERATIONS: u8 = 200;
const CONVERGENCE_THRESHOLD: f64 = 1e-12;

pub(super) struct VincentyDistanceCalculator {
    earth_radius_in_kilometers: f64,
    inverse_flattening_factor: f64,
    semi_minor_axis_in_kilometers: f64,
}

impl VincentyDistanceCalculator {
    pub fn new(
        earth_radius_in_kilometers: f64,
        semi_minor_axis_in_kilometers: f64,
        flattening_factor: f64,
    ) -> Self {
        Self {
            earth_radius_in_kilometers,
            inverse_flattening_factor: flattening_factor,
            semi_minor_axis_in_kilometers,
        }
    }
}

impl DistanceCalculator for VincentyDistanceCalculator {
    fn calculate_distance(
        &self,
        from: &Coordinates,
        to: &Coordinates,
    ) -> Result<f64, DistanceCalculationError> {
        let u1 = ((1. - self.inverse_flattening_factor) * from.latitude.to_radians().tan()).atan();
        let u2 = ((1. - self.inverse_flattening_factor) * to.latitude.to_radians().tan()).atan();
        let l = (to.longitude - from.longitude).to_radians();

        let mut c: f64;
        let mut lambda = l;
        let mut sigma;
        let mut sin_sigma;
        let mut cos_sigma;
        let mut cos_2_sigma_m;
        let mut cos_sq_alpha;

        let mut iterations = 0;
        let mut converged = false;

        loop {
            iterations += 1;

            sin_sigma = ((u2.cos() * lambda.sin()).powi(2)
                + (u1.cos() * u2.sin() - u1.sin() * u2.cos() * lambda.cos()).powi(2))
            .sqrt();

            if sin_sigma == 0. {
                return Ok(0.);
            }

            cos_sigma = u1.sin() * u2.sin() + u1.cos() * u2.cos() * lambda.cos();
            sigma = sin_sigma.atan2(cos_sigma);
            let sin_alpha = u1.cos() * u2.cos() * lambda.sin() / sigma.sin();
            cos_sq_alpha = 1. - sin_alpha.powi(2);
            cos_2_sigma_m = cos_sigma - 2. * u1.sin() * u2.sin() / cos_sq_alpha;

            if cos_2_sigma_m.is_nan() {
                cos_2_sigma_m = 0.;
            }

            c = self.inverse_flattening_factor / 16.
                * cos_sq_alpha
                * (4. + self.inverse_flattening_factor * (4. - 3. * cos_sq_alpha));

            let previous_lambda = lambda;

            lambda = l
                + (1. - c)
                    * self.inverse_flattening_factor
                    * sin_alpha
                    * (sigma
                        + c * sin_sigma
                            * (cos_2_sigma_m + c * cos_sigma * (-1. + 2. * cos_2_sigma_m.powi(2))));

            if (lambda - previous_lambda).abs() < CONVERGENCE_THRESHOLD {
                converged = true;
            }

            if iterations == MAX_ITERATIONS || converged {
                break;
            }
        }

        if !converged {
            return Err(DistanceCalculationError(
                "Failed to converge after 200 iterations".to_string(),
            ));
        }

        let u_sq = cos_sq_alpha
            * (self.earth_radius_in_kilometers.powi(2)
                - self.semi_minor_axis_in_kilometers.powi(2))
            / self.semi_minor_axis_in_kilometers.powi(2);

        let a = 1. + u_sq / 16384. * (4096. + u_sq * (-768. + u_sq * (320. - 175. * u_sq)));
        let b = u_sq / 1024. * (256. + u_sq * (-128. + u_sq * (74. - 47. * u_sq)));
        let delta_sigma = b
            * sin_sigma
            * (cos_2_sigma_m
                + b / 4.
                    * (cos_sigma * (-1. + 2. * cos_2_sigma_m.powi(2))
                        - b / 6.
                            * cos_2_sigma_m
                            * (-3. + 4. * sin_sigma.powi(2))
                            * (-3. + 4. * cos_2_sigma_m.powi(2))));

        Ok(self.semi_minor_axis_in_kilometers * a * (sigma - delta_sigma))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::earth::wgs84;

    use super::*;

    fn assert_calculates_distance_correctly(
        origin: &Coordinates,
        destination: &Coordinates,
        expected_distance: f64,
    ) {
        assert_eq!(
            VincentyDistanceCalculator::new(
                wgs84::EARTH_RADIUS_IN_KILOMETERS,
                wgs84::SEMI_MINOR_AXIS_IN_KILOMETERS,
                wgs84::INVERSE_FLATTENING_FACTOR,
            )
            .calculate_distance(origin, destination)
            .expect("Failed to calculate distance"),
            expected_distance
        )
    }

    #[test]
    fn test_calculates_distance_correctly() {
        let cases = vec![
            ((0.0, 0.0), (0.0, 0.0), 0.),
            ((0.0, 0.0), (0.0, 1.0), 111.31949079322325),
            ((0.0, 0.0), (1.0, 0.0), 110.57438855795696),
            ((0.0, 0.0), (0.5, 179.5), 19936.28857898086),
            (
                (42.3541165, -71.0693514),
                (40.7791472, -73.9680804),
                298.3960574732612,
            ),
        ];

        for ((origin_lat, origin_long), (destination_lat, destination_long), expected_distance) in
            cases
        {
            let origin = Coordinates {
                latitude: origin_lat,
                longitude: origin_long,
            };

            let destination = Coordinates {
                latitude: destination_lat,
                longitude: destination_long,
            };

            assert_calculates_distance_correctly(&origin, &destination, expected_distance)
        }
    }

    #[test]
    fn test_returns_error_if_failed_to_converge() {
        let origin = Coordinates {
            latitude: 0.0,
            longitude: 0.0,
        };

        let destination = Coordinates {
            latitude: 0.5,
            longitude: 179.7,
        };

        let result = VincentyDistanceCalculator::new(
            wgs84::EARTH_RADIUS_IN_KILOMETERS,
            wgs84::SEMI_MINOR_AXIS_IN_KILOMETERS,
            wgs84::INVERSE_FLATTENING_FACTOR,
        )
        .calculate_distance(&origin, &destination);

        assert!(result.is_err());
    }
}
