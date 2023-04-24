use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Apiv2Schema)]
#[serde(rename_all = "lowercase")]
pub enum Datum {
    WGS84,
    NAD27,
    NAD83,
}

impl Default for Datum {
    fn default() -> Self {
        Self::WGS84
    }
}
