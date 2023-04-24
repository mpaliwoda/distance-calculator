use super::{
    airports::AirportsRepository,
    healthcheck::{hostname_provider::HostnameProvider, time_provider::TimeProvider},
};

pub struct AppState {
    pub hostname_provider: Box<dyn HostnameProvider>,
    pub time_provider: Box<dyn TimeProvider>,
    pub airports_repository: Box<dyn AirportsRepository>,
}

impl AppState {
    pub fn new(
        hostname_provider: Box<dyn HostnameProvider>,
        time_provider: Box<dyn TimeProvider>,
        airports_repository: Box<dyn AirportsRepository>,
    ) -> Self {
        Self {
            hostname_provider,
            time_provider,
            airports_repository,
        }
    }
}
