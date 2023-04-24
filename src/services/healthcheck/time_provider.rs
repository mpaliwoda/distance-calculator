use std::time::SystemTime;

pub trait TimeProvider {
    fn now(&self) -> SystemTime;
}

#[cfg(test)]
impl Default for Box<dyn TimeProvider> {
    fn default() -> Self {
        Box::new(SystemTimeProvider::new())
    }
}

pub struct SystemTimeProvider;

impl SystemTimeProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl TimeProvider for SystemTimeProvider {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

#[cfg(test)]
pub struct MockTimeProvider {
    mock_now: SystemTime,
}

#[cfg(test)]
impl MockTimeProvider {
    pub fn new(mock_now: SystemTime) -> Self {
        Self { mock_now }
    }
}

#[cfg(test)]
impl TimeProvider for MockTimeProvider {
    fn now(&self) -> SystemTime {
        self.mock_now.clone()
    }
}
