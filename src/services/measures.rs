use std::collections::{HashMap, LinkedList};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use url::Url;

use crate::entities::perf_result::PerfResult;
use crate::services::fileman::FileMan;

static DEFAULT_MEASURES_FILENAME: &'static str = "measures.json";

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MeasuresData {
    measures: HashMap<url::Url, LinkedList<PerfResult>>,
}

pub struct Measures {
    data: Arc<RwLock<MeasuresData>>,
}

impl Measures {
    pub async fn read_or_default() -> Self {
        let measures_data = FileMan::read_data(DEFAULT_MEASURES_FILENAME)
            .await
            .ok()
            .and_then(|v| serde_json::from_slice::<MeasuresData>(v.as_ref()).ok())
            .unwrap_or_default();
        let data = Arc::new(RwLock::new(measures_data));
        Self { data }
    }

    pub async fn add_measure(&self, url: &Url, measure: PerfResult) {
        let mut data = self.data.write().await;
        data.measures
            .entry(url.to_owned())
            .or_insert_with(Default::default)
            .push_front(measure);
    }

    pub async fn dump(&self) -> anyhow::Result<()> {
        let data = self.data.read().await;
        let measures_data = (*data).clone();
        let bytes = serde_json::to_string_pretty(&measures_data)?;
        FileMan::save_data(DEFAULT_MEASURES_FILENAME, bytes).await?;

        Ok(())
    }
}
