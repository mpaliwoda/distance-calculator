use crate::models::Airport;
use async_trait::async_trait;

mod global_airports_repository;
pub use self::global_airports_repository::GlobalAirportsRepository;

#[async_trait]
pub trait AirportsRepository {
    async fn fetch_airport_by_iata<'a>(
        &self,
        iata_code: &'a str,
    ) -> Result<Option<Airport>, sqlx::Error>;

    async fn unique_airport_iatas<'a>(&self) -> Result<Vec<String>, sqlx::Error>;
}

#[cfg(test)]
pub struct DummyAirportsRepository {
    pub airports: Vec<Airport>,
}

#[cfg(test)]
impl DummyAirportsRepository {
    pub fn new(airports: Vec<Airport>) -> Self {
        Self { airports }
    }
}

#[cfg(test)]
#[async_trait]
impl AirportsRepository for DummyAirportsRepository {
    async fn fetch_airport_by_iata<'a>(
        &self,
        iata_code: &'a str,
    ) -> Result<Option<Airport>, sqlx::Error> {
        let airport = self
            .airports
            .iter()
            .find(|airport| airport.iata_code == iata_code);

        match airport {
            Some(airport) => Ok(Some(airport.to_owned())),
            None => Ok(None),
        }
    }

    async fn unique_airport_iatas<'a>(&self) -> Result<Vec<String>, sqlx::Error> {
        let mut iatas = Vec::new();

        for airport in &self.airports {
            if !iatas.contains(&airport.iata_code) {
                iatas.push(airport.iata_code.to_owned());
            }
        }

        Ok(iatas)
    }
}
