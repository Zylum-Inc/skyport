pub mod chirpstack;

use async_trait::async_trait;
use tokio::sync::broadcast::Sender;

#[async_trait]
pub trait EventSource {
    async fn read_events(&self, output_events: Sender<serde_json::Value>) -> Result<(), anyhow::Error>;
}