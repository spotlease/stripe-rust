use serde::{Deserialize, Serialize};
use super::Metadata;

#[derive(Debug, Default, Serialize)]
pub struct BankAccountCreateParams<'a> {
    pub country: &'a str,
    pub currency: &'a str,
    pub account_holder_name: Option<&'a str>,
    pub account_holder_type: Option<&'a str>,
    pub routing_number: Option<&'a str>,
    pub account_number: &'a str,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Default, Serialize)]
pub struct BankAccountUpdateParams<'a> {
    pub customer: Option<&'a str>,
    pub account_holder_name: Option<&'a str>,
    pub account_holder_type: Option<&'a str>,
    pub default_for_currency: Option<bool>,
    pub metadata: Option<Metadata>,
}

/// The resource representing a Stripe bank account.
///
/// For more details see https://stripe.com/docs/api#customer_bank_account_object.
/// or https://stripe.com/docs/api#account_bank_account_object.
#[derive(Debug, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub account: Option<String>,
    pub account_holder_name: String,
    pub account_holder_type: String,
    pub bank_name: String,
    pub country: String,
    pub currency: String,
    pub customer: Option<String>,
    pub default_for_currency: Option<bool>,
    pub fingerprint: String,
    pub last4: String,
    pub metadata: Metadata,
    pub routing_number: Option<String>,
    pub status: String,
}