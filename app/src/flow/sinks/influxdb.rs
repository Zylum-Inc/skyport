use anyhow::Context;
use influxdb::InfluxDbWriteable;
use serde::Deserialize;
use crate::flow::sinks::EventSink;
use crate::common::chirpstack::{UplinkEvent};
use crate::flow::sinks::adapters::chirpstack::UplinkEventInfluxdbMeasurement;
use anyhow::Result;
use async_trait::async_trait;

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
impl EventSink<UplinkEvent> for InfluxdbSink {
    async fn write_event(&self, event_data: UplinkEvent) -> Result<bool, anyhow::Error> {
        let influx_measurement: UplinkEventInfluxdbMeasurement = event_data.into();

        // Store data in db
        let _ = self
            .write(String::from("uplinks"), influx_measurement)
            .await
            .with_context(|| format!("Unable to write measurement to influxdb"))?;

        Ok(true)
    }
}