use serde::{Deserialize, Serialize};
use crate::models::{Metadata, List, Source, RangeQuery, Timestamp, Address, ExternalAccountParam, Currency, Refund};
use crate::error::ErrorCode;

#[derive(Debug, Deserialize)]
pub struct ChargeOutcome {
    #[serde(rename = "type")]
    pub outcome_type: String, // (authorized, manual_review, issuer_declined, blocked, invalid)
    pub network_status: String, // (approved_by_network, declined_by_network, not_sent_to_network, reversed_after_approval)
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub risk_level: Option<String>, // (normal, elevated, highest, not_assessed, unknown)
    #[serde(default)]
    pub seller_message: Option<String>,
    #[serde(default)]
    pub rule: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FraudDetails {
    pub user_report: Option<String>,
    #[serde(skip_serializing)]
    pub stripe_report: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingDetails {
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>, // eg. Fedex, UPS, USPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// The set of parameters that can be used when capturing a charge.
///
/// For more details see https://stripe.com/docs/api#charge_capture.
#[derive(Default, Serialize)]
pub struct CaptureParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

#[derive(Serialize)]
pub struct DestinationParams<'a> {
    pub account: &'a str,
    pub amount: u64,
}


/// The set of parameters that can be used when creating or updating a charge.
///
/// For more details see https://stripe.com/docs/api#create_charge
#[derive(Default, Serialize)]
pub struct ChargeCreateParams<'a> {
    pub amount: u64,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>, // NOTE: if None, Stripe assumes true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ExternalAccountParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

/// The set of parameters that can be used when creating or updating a charge.
///
/// For more details see https://stripe.com/docs/api#update_charge.
#[derive(Default, Serialize)]
pub struct ChargeUpdateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    All,
    AlipayAccount,
    BankAccount,
    BitcoinReceiver,
    Card,
}

#[derive(Serialize)]
pub struct SourceFilter {
    pub object: SourceType,
}

impl SourceFilter {
    pub fn all() -> SourceFilter {
        SourceFilter { object: SourceType::All }
    }
    pub fn alipay() -> SourceFilter {
        SourceFilter { object: SourceType::AlipayAccount }
    }
    pub fn bank() -> SourceFilter {
        SourceFilter { object: SourceType::BankAccount }
    }
    pub fn bitcoin() -> SourceFilter {
        SourceFilter { object: SourceType::BitcoinReceiver }
    }
    pub fn card() -> SourceFilter {
        SourceFilter { object: SourceType::Card }
    }
}

/// The set of parameters that can be used when listing charges.
///
/// For more details see https://stripe.com/docs/api#list_charges
#[derive(Default, Serialize)]
pub struct ChargeListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The resource representing a Stripe charge.
///
/// For more details see https://stripe.com/docs/api#charges.
#[derive(Debug, Deserialize)]
pub struct Charge {
    pub id: String,
    pub amount: u64,
    pub amount_refunded: u64,
    pub application: Option<String>,
    pub application_fee: Option<String>,
    pub application_fee_amount: Option<u64>,
    pub balance_transaction: Option<String>,
    pub captured: bool,
    pub created: Timestamp,
    pub currency: Currency,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub dispute: Option<String>,
    pub failure_code: Option<ErrorCode>,
    pub failure_message: Option<String>,
    pub fraud_details: FraudDetails,
    pub invoice: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub on_behalf_of: Option<String>,
    pub order: Option<String>,
    pub outcome: Option<ChargeOutcome>,
    pub paid: bool,
    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub refunded: bool,
    pub refunds: List<Refund>,
    pub shipping: Option<ShippingDetails>,
    pub source: Source,
    pub source_transfer: Option<String>,
    pub statement_descriptor: Option<String>,
    pub status: String, // (succeeded, pending, failed)
    pub transfer: Option<String>,
    pub transfer_group: Option<String>,
}