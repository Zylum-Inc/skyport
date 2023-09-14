#![forbid(unsafe_code)]
mod common;
mod config;
mod flow;

use anyhow::{Context, Result};
use config::Config;
use flow::flowgraph::start_single_source_multi_sink_flow;
use flow::sources::chirpstack::event_files::ChirpstackEventFileSource;
use std::path::PathBuf;

use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //Get config path
    let config_path = std::env::args().nth(1).expect("No config file provided");
    let config_file_name = PathBuf::from(config_path);

    //Parse config
    let config: Config = config::Config::new(&config_file_name)
        .with_context(|| format!("Unable to parse config file"))?;

    test(config.chirpstack_source_file_path.clone()).await?;
    Ok(())
}

async fn test(event_file_path: PathBuf) -> Result<(), anyhow::Error> {
    let source = Arc::new(ChirpstackEventFileSource {
        file_path: event_file_path,
    });

    let sink1 = Arc::new(flow::sinks::console::ConsoleSink {});
    let sinks = vec![sink1.clone()];

    start_single_source_multi_sink_flow(source, sinks).await?;
    Ok(())
}
