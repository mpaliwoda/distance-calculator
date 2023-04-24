use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}
