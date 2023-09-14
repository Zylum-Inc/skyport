use crate::common::chirpstack::UplinkEvent;
use crate::flow::sinks::adapters::chirpstack::UplinkEventInfluxdbMeasurement;
use crate::flow::sinks::EventSink;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use influxdb::InfluxDbWriteable;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::broadcast::Receiver;


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
    type EventType = UplinkEvent;
    async fn write_event(&self, event_data: UplinkEvent) -> Result<bool, anyhow::Error> {
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
            let parsed_data: Self::EventType = uplink_event.into();
            let _ = self
                .write_event(parsed_data)
                .await
                .with_context(|| format!("Unable to write measurement to influxdb"))?;
        }
        Ok(())
    }
}
