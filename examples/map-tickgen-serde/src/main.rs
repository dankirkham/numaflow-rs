use numaflow::map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    map::Server::new(TickGen).start().await
}

use chrono::{SecondsFormat, TimeZone, Utc};
use serde::Serialize;

struct TickGen;

#[derive(serde::Deserialize)]
struct Data {
    value: u64,
}

#[derive(serde::Deserialize)]
struct Payload {
    #[serde(rename = "Data")]
    data: Data,
    #[serde(rename = "Createdts")]
    created_ts: i64,
}

#[derive(Serialize)]
struct ResultPayload {
    value: u64,
    time: String,
}

#[tonic::async_trait]
impl map::Mapper for TickGen {
    async fn map(&self, input: map::MapRequest) -> Vec<map::Message> {
        let Ok(payload) = serde_json::from_slice::<Payload>(&input.value) else {
            return vec![];
        };
        let ts = Utc
            .timestamp_nanos(payload.created_ts)
            .to_rfc3339_opts(SecondsFormat::Nanos, true);
        vec![map::Message {
            keys: input.keys,
            value: serde_json::to_vec(&ResultPayload {
                value: payload.data.value,
                time: ts,
            })
            .unwrap_or_default(),
            tags: vec![],
        }]
    }
}
