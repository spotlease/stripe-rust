use super::{Metadata};
use models::{Timestamp, Currency};

/// The set of parameters that can be used when creating or updating a plan.
///
/// For more details see https://stripe.com/docs/api#create_plan and https://stripe.com/docs/api#update_plan.
#[derive(Default, Serialize)]
pub struct PlanParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<&'a str>, // (day, week, month, year)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u64>,
}

/// The resource representing a Stripe plan.
///
/// For more details see https://stripe.com/docs/api#plans.
#[derive(Debug, Deserialize)]
pub struct Plan {
    pub id: String,
    pub amount: u64,
    pub created: Timestamp,
    pub currency: Currency,
    pub interval: String, // (day, week, month, year)
    pub interval_count: u64,
    pub livemode: bool,
    pub metadata: Metadata,
    pub nickname: String,
    pub statement_descriptor: Option<String>,
    pub trial_period_days: Option<u64>,
}