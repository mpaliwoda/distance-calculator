use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Airport {
    pub id: u32,
    pub icao_code: String,
    pub iata_code: String,
    pub name: String,
    pub city: String,
    pub country: String,
    pub lat_deg: i64,
    pub lat_min: i64,
    pub lat_sec: i64,
    pub lat_dir: String,
    pub lon_deg: i64,
    pub lon_min: i64,
    pub lon_sec: i64,
    pub lon_dir: String,
    pub altitude: i64,
    pub lat_decimal: f64,
    pub lon_decimal: f64,
}
