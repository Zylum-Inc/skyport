use crate::flow::sinks::EventSink;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use influxdb::InfluxDbWriteable;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::sync::broadcast::Receiver;

use chrono::{DateTime, Utc};
use crate::common::chirpstack::{RXInfo, UplinkEvent};


#[derive(Debug, Deserialize, Clone)]
pub struct InfluxdbConf {
    pub url: String,
    pub database: String,
    pub token: String,
}

pub struct InfluxdbSink {
    pub conf: InfluxdbConf,
    client: influxdb::Client,
}

impl InfluxdbSink {
    pub fn new(newconf: &InfluxdbConf) -> InfluxdbSink {
        // let client = influxdb::Client::new(conf.url.clone(), conf.database.clone())
        //     .with_auth(conf.username.clone(), conf.password.clone());

        let client = influxdb::Client::new(newconf.url.clone(), newconf.database.clone())
            .with_token(newconf.token.clone());

        return InfluxdbSink {
            conf: newconf.clone(),
            client: client,
        };
    }

    pub async fn write(
        &self,
        name: String,
        data: impl InfluxDbWriteable,
    ) -> Result<String, anyhow::Error> {
        let query = data.into_query(name);
        let write_result = self
            .client
            .query(&query)
            .await
            .with_context(|| format!("Unable to write to influxdb"))?;
        return Ok(write_result);
    }
}

#[async_trait]
impl EventSink for InfluxdbSink {
    async fn write_event(&self, event_data: serde_json::Value) -> Result<bool, anyhow::Error> {
        let influx_measurement: UplinkEventInfluxdbMeasurement = event_data.into();

        // Store data in db
        let _ = self
            .write(String::from("uplinks"), influx_measurement)
            .await
            .with_context(|| format!("Unable to write measurement to influxdb"))?;

        Ok(true)
    }

    async fn write_events(&self, mut input_events: Receiver<Value>) -> Result<(), anyhow::Error> {
        // Process data
        while let Ok(uplink_event) = input_events.recv().await {
            // Store data in db
            let _ = self
                .write_event(uplink_event)
                .await
                .with_context(|| format!("Unable to write measurement to influxdb"))?;
        }
        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize, InfluxDbWriteable)]
pub struct UplinkEventInfluxdbMeasurement {
    pub time: DateTime<Utc>,

    pub highest_rssi: i32,

    pub latitude: f64,

    pub longitude: f64,

    #[influxdb(tag)]
    pub device_name: String,

    #[influxdb(tag)]
    pub dev_eui: String,
}

impl From<UplinkEvent> for UplinkEventInfluxdbMeasurement {
    fn from(event: UplinkEvent) -> Self {
        //Find the rxinfo with highest rssi
        let mut highest_rssi_receiver: RXInfo = event.rxInfo[0].clone();
        for rxinfo in event.rxInfo {
            if rxinfo.rssi > highest_rssi_receiver.rssi {
                highest_rssi_receiver = rxinfo;
            }
        }

        return UplinkEventInfluxdbMeasurement {
            time: event.time,
            highest_rssi: highest_rssi_receiver.rssi,
            latitude: highest_rssi_receiver.location.latitude,
            longitude: highest_rssi_receiver.location.longitude,
            device_name: event.deviceInfo.deviceName,
            dev_eui: event.deviceInfo.devEui,
        };
    }
}

impl From<Value> for UplinkEventInfluxdbMeasurement {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
