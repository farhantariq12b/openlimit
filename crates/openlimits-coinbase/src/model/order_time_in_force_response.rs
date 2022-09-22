use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;
use super::shared::naive_datetime_from_string;

/// This enum represents a response of a time in force
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForceResponse {
    GTC,
    GTT {
        #[serde(with = "naive_datetime_from_string")]
        expire_time: NaiveDateTime,
    },
    IOC,
    FOK,
}