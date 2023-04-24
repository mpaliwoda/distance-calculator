use std::{ffi::OsString, io};

pub trait HostnameProvider {
    fn get(&self) -> io::Result<OsString>;
}

#[cfg(test)]
impl Default for Box<dyn HostnameProvider> {
    fn default() -> Self {
        Box::new(HostnameCrateHostnameProvider::new())
    }
}

pub struct HostnameCrateHostnameProvider;

impl HostnameCrateHostnameProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl HostnameProvider for HostnameCrateHostnameProvider {
    fn get(&self) -> io::Result<OsString> {
        hostname::get()
    }
}

#[cfg(test)]
pub struct MockSuccessfulHostnameProvider {
    mock_hostname: OsString,
}

#[cfg(test)]
impl MockSuccessfulHostnameProvider {
    pub fn new(mock_hostname: OsString) -> Self {
        Self { mock_hostname }
    }
}

#[cfg(test)]
impl HostnameProvider for MockSuccessfulHostnameProvider {
    fn get(&self) -> io::Result<OsString> {
        Ok(self.mock_hostname.clone())
    }
}

#[cfg(test)]
pub struct MockFailedHostnameProvider;

#[cfg(test)]
impl MockFailedHostnameProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
impl HostnameProvider for MockFailedHostnameProvider {
    fn get(&self) -> io::Result<OsString> {
        Err(io::Error::new(io::ErrorKind::Other, "mock error"))
    }
}
