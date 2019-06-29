use serde::{Deserialize, Serialize};
use super::{Metadata, Address, Timestamp};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeclineChargeSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PayoutSchedule {
    pub delay_days: u64,
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TOSAcceptanceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BusinessType {
    Individual,
    Company
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateOfBirth {
    year: u64,
    month: u64,
    day: u64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationDocumentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentParams>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParams>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_primary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_url: Option<String>,
    pub charges_enabled: bool,
    pub country: String,
    pub debit_negative_balances: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_charge_on: Option<DeclineChargeSettings>,
    pub default_currency: String,
    pub details_submitted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    // pub external_accounts: List<BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,
    pub metadata: Metadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_schedule: Option<PayoutSchedule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_statement_descriptor: Option<String>,
    pub payouts_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    pub timezone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TOSAcceptanceDetails>,
    #[serde(rename = "type")]
    pub account_type: String, // (Stripe, Custom, or Express)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<serde_json::Value>,
}

#[derive(Debug, Default, Serialize)]
pub struct AccountParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TOSAcceptanceDetails>,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Standard,
    Custom
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Standard
    }
}

/// The set of parameters that can be used when creating an account for users.
///
/// For more details see https://stripe.com/docs/api#create_account.
#[derive(Debug, Default, Serialize)]
pub struct AccountCreateParams<'a> {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(flatten)]
    pub account: AccountParams<'a>,
    
}

#[derive(Debug, Default, Serialize)]
pub struct AccountUpdateParams<'a> {
    #[serde(flatten)]
    pub account: AccountParams<'a>,
}

#[derive(Default, Serialize)]
pub struct AccountListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}