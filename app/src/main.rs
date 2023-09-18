#![forbid(unsafe_code)]
mod common;
mod config;
mod flow;

use anyhow::{Context, Result};
use config::Config;
use flow::flowgraph::start_single_source_multi_sink_flow;
use flow::sinks::EventSink;
use flow::sources::chirpstack::event_files::ChirpstackEventFileSource;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //Get config path
    let config_path = std::env::args().nth(1).expect("No config file provided");
    let config_file_name = PathBuf::from(config_path);

    //Parse config
    let config: Config = config::Config::new(&config_file_name)
        .with_context(|| format!("Unable to parse config file"))?;

    test(&config).await?;
    Ok(())
}

async fn test(config: &Config) -> Result<(), anyhow::Error> {
    let source = ChirpstackEventFileSource {
        file_path: config.source_chirpstack_events_file.clone(),
    };
    let source = Box::new(source);

    let sink1 = flow::sinks::console::ConsoleSink {};
    let sink1: Box<dyn EventSink + Send + Sync + 'static> = Box::new(sink1);

    let sink2 = flow::sinks::pg::PostgresSink::new(
        config.sink_pg.clone().unwrap(),
    ).await;
    let sink2: Box<dyn EventSink + Send + Sync + 'static>  = Box::new(sink2);
    
    let sinks = vec![sink1, sink2];

    start_single_source_multi_sink_flow(source, sinks).await?;
    Ok(())
}
