use serde::{Deserialize, Serialize};
use super::{Currency, ExternalAccountParam, List, Metadata, RangeQuery, Source, Timestamp};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CustomerSourceParam<'a> {
    Id(&'a str),
    Token(&'a str),
    Card(ExternalAccountParam<'a>),
}

/// The set of parameters that can be used when creating or updating a customer.
///
/// For more details see https://stripe.com/docs/api#create_customer and https://stripe.com/docs/api#update_customer.
#[derive(Default, Serialize)]
pub struct CustomerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "default_source")]
    pub default_source_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CustomerSourceParam<'a>>,
}

/// The set of parameters that can be used when listing customers.
///
/// For more details see https://stripe.com/docs/api#list_customers
#[derive(Default, Serialize)]
pub struct CustomerListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

/// The resource representing a Stripe customer.
///
/// For more details see https://stripe.com/docs/api#customers.
#[derive(Debug, Deserialize)]
pub struct Customer {
    pub id: String,
    pub balance: i64,
    pub created: u64,
    pub currency: Option<Currency>,
    #[serde(rename = "default_source")]
    pub default_source_id: Option<String>,
    pub email: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub sources: List<Source>,
}
