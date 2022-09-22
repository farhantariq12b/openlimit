use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::OrderTimeInForceResponse;
use super::shared::string_to_decimal;

/// This enum represents the order types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OrderType {
    Limit {
        #[serde(with = "string_to_decimal")]
        size: Decimal,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        #[serde(flatten)]
        time_in_force: OrderTimeInForceResponse,
    },
    Market {
        #[serde(default)]
        #[serde(with = "string_to_decimal")]
        size: Decimal,
        #[serde(default)]
        #[serde(with = "string_to_decimal")]
        funds: Decimal,
    },
}