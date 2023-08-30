mod common;
mod flow;

use crate::common::chirpstack::{ChirpstackEvents};
use crate::flow::sinks::adapters::chirpstack::UplinkEventInfluxdbMeasurement;
use crate::flow::sinks::influxdb::{InfluxdbConf, InfluxdbSink};
use anyhow::{Context, Result};
use flow::sources::chirpstack::event_files::read_events_file;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    test().await?;
    Ok(())
}

async fn test() -> Result<(), anyhow::Error> {

    //Parse input argument as the filename
    let input_path = std::env::args().nth(1).expect("No input file provided");
    let input_file_name = PathBuf::from(input_path);

    //Parse second input as destination host and port
    let influxdb_host = std::env::args().nth(2).expect("No influxdb host:port provided");

    // Parse file for data
    let events = read_events_file(&input_file_name)
        .with_context(|| format!("Chirpstack Events could not be parsed"))?;

    println!("Events found : {}", events.len());

    //Prepare Influxdb sink
    let influxdb_conf = InfluxdbConf {
        url: format!("http://{}", influxdb_host).to_string(),
        database: "default_bucket".to_string(),
        token: "no_secrets".to_string()
    };
    let influxdb_sink: InfluxdbSink = InfluxdbSink::new(&influxdb_conf);

    // Process data
    for event in events {
        match event {
            ChirpstackEvents::UPLINK(uplink) => {
                let influx_measurement: UplinkEventInfluxdbMeasurement = uplink.into();

                // Store data in db
                // println!("Influx Measurement: {:?}", influx_measurement);
                let write_res = influxdb_sink
                    .write(String::from("uplinks"), influx_measurement)
                    .await
                    .with_context(|| format!("Unable to write measurement to influxdb"))?;
                println!("Write result: {:?}", write_res);
            }
            _ => {
                println!("Ignored unknown event type");
            }
        }
    }

    Ok(())
}
