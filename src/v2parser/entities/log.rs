use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Log {
    #[serde(rename = "logLevel")]
    log_level: LogLevel,

    #[serde(rename = "dnsLog")]
    dns_log: bool,
}

impl Log {
    #[allow(unused)]
    pub fn with_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }

    #[allow(unused)]
    pub fn with_dns_log(&mut self, dns_log: bool) {
        self.dns_log = dns_log;
    }
}

impl Default for Log {
    fn default() -> Self {
        Self {
            log_level: Default::default(),
            dns_log: false,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Copy, Serialize, Deserialize)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum LogLevel {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "debug")]
    Debug,

    #[serde(rename = "info")]
    Info,

    #[serde(rename = "warning")]
    Warning,

    #[serde(rename = "error")]
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::None
    }
}
