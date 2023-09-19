use super::EventSink;
use crate::common::chirpstack::{ChirpstackEvents, UplinkEvent};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::broadcast::Receiver;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostgresConf {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostgresEventRecord {
    pub event_id: uuid::Uuid,
    pub event_time: chrono::DateTime<chrono::Utc>,
    pub device_id: String,
    pub device_name: String,
    pub event_type: String,
    pub event_data: serde_json::Value,
}

impl From<Value> for PostgresEventRecord {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl From<UplinkEvent> for PostgresEventRecord {
    fn from(event: UplinkEvent) -> Self {
        PostgresEventRecord {
            event_id: event.deduplicationId,
            event_time: event.time,
            device_id: event.deviceInfo.devEui.clone(),
            device_name: event.deviceInfo.deviceName.clone(),
            event_type: "UPLINK".to_string(),
            event_data: serde_json::to_value(event).unwrap(),
        }
    }
}

pub struct PostgresSink {
    pub config: PostgresConf,
    pool: PgPool,
}

impl PostgresSink {
    pub async fn new(config: PostgresConf) -> Self {
        //Create a connection pool
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!(
                "postgres://{}:{}@{}:{}/{}",
                config.user, config.password, config.host, config.port, config.database
            ))
            .await
            .expect("Failed to create pool");
        let sink = PostgresSink { config, pool };

        //Create Necessary Tables
        let create_schema_q = Self::prepare_device_events_table_schema();
        create_schema_q.execute(&sink.pool).await.unwrap();
        sink
    }

    fn prepare_device_events_table_schema(
    ) -> sqlx::query::Query<'static, sqlx::Postgres, sqlx::postgres::PgArguments> {
        let q = sqlx::query(
            "CREATE TABLE IF NOT EXISTS device_events (
            event_id UUID PRIMARY KEY,
            event_time TIMESTAMPTZ NOT NULL,
            device_id varchar NOT NULL,
            device_name varchar NOT NULL,
            event_type varchar NOT NULL,
            event_data JSONB NULL
        )",
        );
        q
    }

    fn prepare_pg_query(
        &self,
        event_data: PostgresEventRecord,
    ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        let event_id = event_data.event_id;
        let event_time = event_data.event_time;
        let device_id = event_data.device_id;
        let device_name = event_data.device_name;
        let event_type = event_data.event_type;
        let event_data = serde_json::to_value(event_data.event_data).unwrap();

        let q = sqlx::query(
            "INSERT INTO device_events (event_id, event_time, device_id, device_name, event_type, event_data) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(event_id)
        .bind(event_time)
        .bind(device_id)
        .bind(device_name)
        .bind(event_type)
        .bind(event_data);
        q
    }
}

#[async_trait]
impl EventSink for PostgresSink {
    async fn write_event(&self, event_data: serde_json::Value) -> Result<bool, anyhow::Error> {
        let q = self.prepare_pg_query(event_data.into());
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected() > 0)
    }

    async fn write_events(
        &self,
        mut input_events: Receiver<serde_json::Value>,
    ) -> Result<(), anyhow::Error> {
        while let Ok(event) = input_events.recv().await {
            // println!("Event JSON Received: {:?}", event.to_string());
            let event_data: ChirpstackEvents = event.into(); //Fix this hack. Use an enum+Value ?
            match event_data {
                ChirpstackEvents::UPLINK(uplink_event) => {
                    let event_data: PostgresEventRecord = uplink_event.into();
                    let q = self.prepare_pg_query(event_data);
                    let res = q.execute(&self.pool).await;
                    match res {
                        Ok(qres) => {
                            // println!("PgQueryResult: {:?}", qres);
                            if qres.rows_affected() == 0 {
                                eprintln!("Failed to write event to Postgres");
                            }
                        }
                        Err(err) => {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                }
                _ => {
                    println!("Ignoring unsupported event while writing to PG");
                }
            }
        }
        Ok(())
    }
}
