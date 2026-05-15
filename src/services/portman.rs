use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[must_use]
pub(crate) struct PortLease<F>(u16, F)
where
    F: FnMut();

impl<F> PortLease<F>
where
    F: FnMut(),
{
    fn new(port: u16, on_drop: F) -> Self {
        PortLease(port, on_drop)
    }

    pub fn port(&self) -> u16 {
        self.0
    }
}

impl<F> Drop for PortLease<F>
where
    F: FnMut(),
{
    fn drop(&mut self) {
        self.1();
    }
}

pub(crate) struct Portman {
    ports: Arc<Mutex<HashSet<u16>>>,
}

impl Portman {
    pub fn new() -> Portman {
        Self {
            ports: Arc::default(),
        }
    }

    fn hold(&self, port: u16) -> anyhow::Result<()> {
        let mut ports = match self.ports.lock() {
            Ok(ports) => ports,
            Err(e) => e.into_inner(),
        };
        if ports.contains(&port) {
            anyhow::bail!("Port already taken")
        }
        ports.insert(port);
        Ok(())
    }

    fn get_random_port() -> u16 {
        rand::random_range(10000..65535)
    }

    fn hold_random(&self) -> PortLease<impl FnMut()> {
        let mut random = Self::get_random_port();
        while self.hold(random).is_err() {
            random = Self::get_random_port();
        }

        let ports = self.ports.clone();
        PortLease::new(random, move || {
            let mut ports = match ports.lock() {
                Ok(ports) => ports,
                Err(e) => e.into_inner(),
            };
            ports.remove(&random);
        })
    }

    pub async fn lease_port<F, Fut, R>(&self, doer: F) -> R
    where
        Fut: Future<Output = R>,
        F: FnOnce(u16) -> Fut,
    {
        let lease = self.hold_random();
        let result = doer(lease.port()).await;

        result
    }
}
