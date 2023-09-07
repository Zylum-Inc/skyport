use async_trait::async_trait;

pub mod influxdb;
pub mod adapters;

#[async_trait]
pub trait EventSink<T> {
    async fn write_event(&self, event_data: T) -> Result<bool, anyhow::Error>;
}