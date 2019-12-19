use serde::{Deserialize, Serialize};
use crate::models::{Metadata, RangeQuery, Timestamp, Currency};
use crate::error::ErrorCode;

/// The set of parameters that can be used when creating or updating a charge.
///
/// For more details see https://stripe.com/docs/api#create_payout
#[derive(Default, Serialize)]
pub struct PayoutCreateParams<'a> {
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

/// The set of parameters that can be used when creating or updating a payout.
///
/// For more details see https://stripe.com/docs/api#update_payout.
#[derive(Default, Serialize)]
pub struct PayoutUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// The set of parameters that can be used when listing payouts.
///
/// For more details see https://stripe.com/docs/api#list_payouts
#[derive(Default, Serialize)]
pub struct PayoutListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

/// The resource representing a Payout.
///
/// For more details see https://stripe.com/docs/api#payouts.
#[derive(Debug, Deserialize)]
pub struct Payout {
    pub id: String,
    pub amount: u64,
    pub arrival_date: Option<Timestamp>,
    pub automatic: Option<bool>,
    pub balance_transaction: Option<String>,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub failure_code: Option<ErrorCode>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub method: Option<String>,
    pub source_type: Option<String>,
    pub statement_descriptor: Option<String>,
    pub status: String,
    pub r#type: Option<String>
}