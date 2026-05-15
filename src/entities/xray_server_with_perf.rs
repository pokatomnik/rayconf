use std::fmt::Display;

use crate::entities::{perf_result::PerfResult, xray_server::XRayServer};

pub(crate) struct XRayServerWithPerf(XRayServer, Option<PerfResult>);

impl XRayServerWithPerf {
    pub fn new(xray_server: XRayServer, duration: Option<PerfResult>) -> Self {
        Self(xray_server, duration)
    }

    #[allow(unused)]
    pub fn xray_server(&self) -> &XRayServer {
        &self.0
    }

    #[allow(unused)]
    pub fn duration(&self) -> Option<PerfResult> {
        return self.1.clone();
    }
}

impl Display for XRayServerWithPerf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let server_name = self.0.to_string();
        let duration = self
            .1
            .clone()
            .and_then(|v| v.perf_data())
            .map(|v| format!("{} mbps", v.download_speed_mbps()))
            .unwrap_or_else(|| "n/a".to_string());
        let title = format!("{}, {}", server_name, duration);
        f.write_str(title.as_str())
    }
}
