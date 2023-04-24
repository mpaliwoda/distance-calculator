use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Apiv2Schema)]
#[serde(rename_all = "snake_case")]
pub enum Formula {
    GreatCircle,
    Haversine,
    Vincenty,
}

impl Default for Formula {
    fn default() -> Self {
        Self::GreatCircle
    }
}
