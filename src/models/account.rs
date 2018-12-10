use super::{Metadata, NullableOption};
use models::Timestamp;

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
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub legal_entity: Option<json::Value>,
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
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub verification: Option<json::Value>,
}

/// The set of parameters that can be used when creating an account for users.
///
/// For more details see https://stripe.com/docs/api#create_account.
#[derive(Debug, Default, Serialize)]
pub struct AccountCreateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(rename = "type")]
    pub account_type: AccountType
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

#[derive(Debug, Default, Serialize)]
pub struct AccountUpdateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_url: NullableOption<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TOSAcceptanceDetails>,
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