pub mod adapters;
pub mod influxdb;
pub mod pg;
pub mod console;

use async_trait::async_trait;
use tokio::sync::broadcast::Receiver;

#[async_trait]
pub trait EventSink {
    type EventType: From<serde_json::Value>;
    async fn write_event(&self, event_data: Self::EventType) -> Result<bool, anyhow::Error>;
    async fn write_events(&self, mut input_events: Receiver<serde_json::Value>) -> Result<(), anyhow::Error>;
}
