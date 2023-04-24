use async_trait::async_trait;
use moka::future::Cache;
use sqlx::{Row, SqlitePool};

use crate::models::Airport;

use super::AirportsRepository;

#[derive(Clone)]
pub struct GlobalAirportsRepository {
    pool: SqlitePool,
    airports_cache: Cache<String, Option<Airport>>,
    iatas_cache: Cache<(), Vec<String>>,
}

impl GlobalAirportsRepository {
    pub async fn new(pool: SqlitePool) -> Self {
        let airports_cache = Cache::new(9300);
        let iatas_cache = Cache::new(1);

        Self {
            pool,
            airports_cache,
            iatas_cache,
        }
    }
}

#[async_trait]
impl AirportsRepository for GlobalAirportsRepository {
    async fn fetch_airport_by_iata<'a>(
        &self,
        iata_code: &'a str,
    ) -> Result<Option<Airport>, sqlx::Error> {
        match self.airports_cache.get(iata_code) {
            Some(airport) => return Ok(airport),
            None => {
                let query = "SELECT * FROM airports WHERE iata_code = ? AND name != 'N/A'";
                let airport = sqlx::query_as::<_, Airport>(query)
                    .bind(iata_code)
                    .fetch_optional(&self.pool)
                    .await?;

                self.airports_cache
                    .insert(iata_code.to_owned(), airport.clone())
                    .await;

                Ok(airport)
            }
        }
    }

    async fn unique_airport_iatas<'a>(&self) -> Result<Vec<String>, sqlx::Error> {
        match self.iatas_cache.get(&()) {
            Some(iatas) => return Ok(iatas),
            None => {
                let query = "SELECT DISTINCT iata_code FROM airports WHERE name != 'N/A'";
                let iatas = sqlx::query(query).fetch_all(&self.pool).await?;

                let iatas = iatas
                    .iter()
                    .map(|row| row.get::<String, _>("iata_code"))
                    .collect::<Vec<String>>();

                self.iatas_cache.insert((), iatas.clone()).await;

                Ok(iatas)
            }
        }
    }
}
