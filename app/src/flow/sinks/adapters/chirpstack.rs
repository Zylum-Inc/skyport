use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;

use crate::common::chirpstack::{RXInfo, UplinkEvent};

#[derive(Debug, InfluxDbWriteable)]
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
