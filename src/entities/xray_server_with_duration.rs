use std::{fmt::Display, time::Duration};

use crate::entities::xray_server::XRayServer;

pub(crate) struct XRayServerWithDuration(XRayServer, Option<Duration>);

impl XRayServerWithDuration {
    pub fn new(xray_server: XRayServer, duration: Option<Duration>) -> Self {
        Self(xray_server, duration)
    }

    #[allow(unused)]
    pub fn xray_server(&self) -> &XRayServer {
        &self.0
    }

    #[allow(unused)]
    pub fn duration(&self) -> Option<Duration> {
        return self.1;
    }
}

impl Display for XRayServerWithDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let server_name = self.0.to_string();
        let duration = self
            .1
            .map(|v| format!("{} ms", v.as_millis()))
            .unwrap_or_else(|| "n/a".to_string());
        let title = format!("{}, {}", server_name, duration);
        f.write_str(title.as_str())
    }
}
