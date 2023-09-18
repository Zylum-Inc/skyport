use crate::{common::chirpstack::ChirpstackEvents, flow::sources::EventSource};
use anyhow::Context;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::{fs::File, io::BufReader, path::PathBuf};
use tokio::sync::broadcast::Sender;

#[derive(Debug, Deserialize)]
pub struct ChirpstackEventFileSource {
    pub file_path: PathBuf,
}

impl ChirpstackEventFileSource {
    pub fn read_events_file(&self) -> Result<Vec<ChirpstackEvents>, anyhow::Error> {
        // read file and parse into config struct
        let file = File::open(&self.file_path)
            .with_context(|| format!("Unable to open events file: {}", self.file_path.display()))?;
        let reader = BufReader::new(file);
        let events: Vec<ChirpstackEvents> =
            serde_json::from_reader(reader).expect("Unable to read or parse events file");
        return Ok(events);
    }
}

#[async_trait]
impl EventSource for ChirpstackEventFileSource {
    
    async fn read_events(
        &self,
        output_events: Sender<Value>,
    ) -> Result<(), anyhow::Error> {
        let events = self
            .read_events_file()
            .with_context(|| format!("Chirpstack Events could not be parsed"))?;
        for event in events {
            let data: Value = event.into();
            // println!("Sending: {:?}", data);
            output_events.send(data)?;
            // break;
        }
        Ok(())
    }
}
