use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use url::Url;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PerfData {
    download_speed_mbps: u64,
}

impl PerfData {
    pub fn from_raw_data(bytes_downloaded: u64, time_taken: Duration) -> Self {
        let download_speed_mbps = bytes_downloaded * 8 / time_taken.as_secs() / 1_000_000;
        Self {
            download_speed_mbps,
        }
    }

    #[allow(unused)]
    pub fn new(download_speed_mbps: u64) -> Self {
        Self {
            download_speed_mbps,
        }
    }

    #[allow(unused)]
    pub fn download_speed_mbps(&self) -> u64 {
        return self.download_speed_mbps;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PerfResult {
    url: url::Url,

    #[serde(with = "approx_instant")]
    date: Instant,

    perf_data: Option<PerfData>,
}

impl PerfResult {
    pub fn now(url: &Url, perf_data: Option<PerfData>) -> Self {
        Self {
            url: url.to_owned(),
            date: Instant::now(),
            perf_data,
        }
    }

    #[allow(unused)]
    pub fn url(&self) -> &url::Url {
        &self.url
    }

    #[allow(unused)]
    pub fn date(&self) -> Instant {
        self.date
    }

    #[allow(unused)]
    pub fn perf_data(&self) -> Option<PerfData> {
        return self.perf_data;
    }
}

mod approx_instant {
    use std::time::SystemTime;

    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
    use tokio::time::Instant;

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let system_now = SystemTime::now();
        let instant_now = Instant::now();
        let approx = system_now - (instant_now - *instant);
        approx.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de = SystemTime::deserialize(deserializer)?;
        let system_now = SystemTime::now();
        let instant_now = Instant::now();
        let duration = system_now.duration_since(de).map_err(Error::custom)?;
        let approx = instant_now - duration;
        Ok(approx)
    }
}
