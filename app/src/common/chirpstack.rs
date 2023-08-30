use chrono::{DateTime, Utc};
use serde::{Deserialize};
use serde_json::{Map, Value};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChirpstackEvents {
    UPLINK(UplinkEvent),
    STATUS(StatusEvent),
    JOIN(JoinEvent),
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceInfo {
    pub tenantId: Uuid,
    pub tenantName: String,
    pub applicationId: Uuid,
    pub applicationName: String,
    pub deviceProfileId: Uuid,
    pub deviceProfileName: String,
    pub deviceName: String,
    pub devEui: String,
    pub tags: Map<String, Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum LocationSource {
    // Unknown.
    UNKNOWN = 0,

    // GPS.
    GPS = 1,

    // Manually configured.
    CONFIG = 2,

    // Geo resolver (TDOA).
    GEO_RESOLVER_TDOA = 3,

    // Geo resolver (RSSI).
    GEO_RESOLVER_RSSI = 4,

    // Geo resolver (GNSS).
    GEO_RESOLVER_GNSS = 5,

    // Geo resolver (WIFI).
    GEO_RESOLVER_WIFI = 6,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub source: Option<LocationSource>,
    pub accuracy: Option<f32>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum CRCStatus {
    // No CRC.
    NO_CRC = 0,

    // Bad CRC.
    BAD_CRC = 1,

    // CRC OK.
    CRC_OK = 2,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct RXInfo {
    pub gatewayId: String,
    pub uplinkId: u32,
    pub rssi: i32,
    pub snr: f32,
    pub channel: Option<u32>,
    pub rfChain: Option<u32>,
    pub location: Location,
    pub context: String,
    pub metadata: Map<String, Value>,
    pub crcStatus: CRCStatus,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CodeRate {
    CR_UNDEFINED = 0,
    CR_4_5 = 1, // LoRa
    CR_4_6 = 2,
    CR_4_7 = 3,
    CR_4_8 = 4,
    CR_3_8 = 5, // LR-FHSS
    CR_2_6 = 6,
    CR_1_4 = 7,
    CR_1_6 = 8,
    CR_5_6 = 9,
    CR_LI_4_5 = 10, // LoRa 2.4 gHz
    CR_LI_4_6 = 11,
    CR_LI_4_8 = 12,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LoraModulationInfo {
    pub bandwidth: u32,
    pub spreadingFactor: u32,
    pub codeRateLegacy: Option<String>,
    pub polarizationInversion: Option<bool>,
    pub codeRate: CodeRate,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FskModulationInfo {
    pub frequencyDeviation: u32,
    pub datarate: u32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LrFhssModulationInfo {
    pub operatingChannelWidth: u32,
    pub codeRateLegacy: String,
    pub codeRate: CodeRate,
    pub gridSteps: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Modulation {
    lora(LoraModulationInfo),
    fsk(FskModulationInfo),
    lr_fhss(LrFhssModulationInfo),
}

#[derive(Debug, Deserialize)]
pub struct TXInfo {
    pub frequency: u32,
    pub modulation: Modulation,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct UplinkEvent {
    pub deduplicationId: Uuid,
    pub time: DateTime<Utc>,
    pub deviceInfo: DeviceInfo,
    pub devAddr: String,
    pub adr: bool,
    pub dr: u8,
    pub fCnt: u32,
    pub fPort: u8,
    pub confirmed: bool,
    pub data: String,
    pub rxInfo: Vec<RXInfo>,
    pub txInfo: TXInfo,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct StatusEvent {
    pub deduplicationId: Uuid,
    pub time: DateTime<Utc>,
    pub deviceInfo: DeviceInfo,
    pub margin: i32,
    pub externalPowerSource: bool,
    pub batteryLevelUnavailable: bool,
    pub batteryLevel: f32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RelayRxInfo {
    pub devEui: String,
    pub frequency: u32,
    pub dr: u32,
    pub snr: i32,
    pub rssi: i32,
    pub worChannel: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct JoinEvent {
    pub deduplicationId: Uuid,
    pub time: DateTime<Utc>,
    pub deviceInfo: DeviceInfo,
    pub devAddr: String,
    pub relayRxInfo: Option<RelayRxInfo>,
}
