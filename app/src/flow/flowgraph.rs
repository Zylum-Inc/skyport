use super::sinks::EventSink;
use super::sources::EventSource;
use std::sync::Arc;

// Single Source Multi Sink Flow
pub async fn start_single_source_multi_sink_flow<I, O>(
    event_source: Arc<I>,
    event_sinks: Vec<Arc<O>>,
) -> Result<(), anyhow::Error>
where
    I: EventSource + Send + Sync + 'static,
    O: EventSink + Send + Sync + 'static,
{
    //Create a tokio channel
    let (tx, _rx) = tokio::sync::broadcast::channel::<serde_json::Value>(1000);

    //Spawn a tokio task for each sink
    let mut handles = Vec::new();
    for sink in event_sinks.into_iter() {
        let rx_sub = tx.subscribe();
        let handle = tokio::task::spawn(async move { sink.write_events(rx_sub).await });
        handles.push(handle);
    }

    //Spawn a tokio task for the source
    let source_handle = tokio::task::spawn(async move { event_source.read_events(tx).await });

    //Wait for the source and sinks to finish
    let _ = source_handle.await?;
    for handle in handles {
        let _ = handle.await?;
    }
    Ok(())
}
