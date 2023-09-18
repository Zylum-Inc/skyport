use super::EventSink;
use async_trait::async_trait;
use tokio::sync::broadcast::{Receiver, error::RecvError};
pub struct ConsoleSink {}

#[async_trait]
impl EventSink for ConsoleSink {
    async fn write_event(&self, event_data: serde_json::Value) -> Result<bool, anyhow::Error> {
        println!("ConsoleSink: {:?}", event_data);
        return Ok(true);
    }

    async fn write_events(
        &self,
        mut input_events: Receiver<serde_json::Value>,
    ) -> Result<(), anyhow::Error> {
        loop {
            match input_events.recv().await {
                Ok(val) => {
                    println!("ConsoleSink: Event Received => {}", val);
                },
                Err(RecvError::Closed) => {
                    println!("ConsoleSink: Channel Closed");
                    return Ok(());
                },
                Err(RecvError::Lagged(_)) => {
                    println!("ConsoleSink: Channel Lagged");
                    return Ok(());
                },
            }
        }
    }
}
