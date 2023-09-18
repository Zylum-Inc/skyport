use anyhow::{Context};
use serde::{Deserialize};
use std::{fs::File, io::BufReader, path::PathBuf};
use crate::flow::sinks::pg::PostgresConf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub source_chirpstack_events_file: PathBuf,
    pub sink_pg: Option<PostgresConf>,
}

impl Config {
    pub fn new(file_path: &PathBuf) -> Result<Config, anyhow::Error> {
        // read file and parse into config struct
        let file = File::open(file_path)
            .with_context(|| format!("Unable to open config file: {}", file_path.display()))?;
        let reader = BufReader::new(file);
        let conf: Config =
            serde_json::from_reader(reader).expect("Unable to read or parse config file");
        return Ok(conf);
    }
}
