use crate::common::chirpstack::{ChirpstackEvents};
use anyhow::Context;
use std::{fs::File, io::BufReader, path::PathBuf};

pub fn read_events_file(file_path: &PathBuf) -> Result<Vec<ChirpstackEvents>, anyhow::Error> {
    // read file and parse into config struct
    let file = File::open(file_path)
        .with_context(|| format!("Unable to open events file: {}", file_path.display()))?;
    let reader = BufReader::new(file);
    let events: Vec<ChirpstackEvents> =
        serde_json::from_reader(reader).expect("Unable to read or parse events file");
    return Ok(events);
}
