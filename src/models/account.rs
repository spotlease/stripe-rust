use serde::{Deserialize, Serialize};
use super::{Metadata, Timestamp, Address, Person, Weekday, Currency, Dob, DelayDays};



#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BusinessProfile {
    /// The merchant category code for the account.
    ///
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,

    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Internal-only description of the product sold or service provided by the business.
    ///
    /// It's used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<Address>,

    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,

    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,

    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,

    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilities {
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilityStatus>,

    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CapabilityStatus>,

    /// The status of the platform payments capability of the account, or whether your platform can process charges on behalf of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_payments: Option<CapabilityStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountRequirements {
    /// The date the fields in `currently_due` must be collected by to keep payouts enabled for the account.
    ///
    /// These fields might block payouts sooner if the next threshold is reached before these fields are collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<Timestamp>,

    /// The fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by the `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Vec<String>>,

    /// If the account is disabled, this string describes why the account can’t create charges or receive payouts.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `rejected.fraud`, `rejected.terms_of_service`, `rejected.listed`, `rejected.other`, `listed`, `under_review`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,

    /// The fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, these fields appear in `currently_due` as well, and the `current_deadline` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Vec<String>>,

    /// The fields that weren't collected by the `current_deadline`.
    ///
    /// These fields need to be collected to re-enable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountSettings {
    pub branding: BrandingSettings,

    pub card_payments: CardPaymentsSettings,

    pub dashboard: DashboardSettings,

    pub payments: PaymentsSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettings>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettings {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    ///
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    ///
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOn>,

    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DashboardSettings {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOn {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    ///
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    pub avs_failure: bool,

    /// Whether Stripe automatically declines charges with an incorrect CVC.
    ///
    /// This setting only applies when a CVC is provided and it fails bank verification.
    pub cvc_failure: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettings {
    /// The default text that appears on credit card statements when a charge is made.
    ///
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<String>,

    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutSettings {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// See the [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details.
    /// Default value is `true` for Express accounts and `false` for Custom accounts.
    pub debit_negative_balances: bool,

    pub schedule: TransferSchedule,

    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TosAcceptance {
    /// The Unix timestamp marking when the Stripe Services Agreement was accepted by the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    /// The IP address from which the Stripe Services Agreement was accepted by the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the Stripe Services Agreement was accepted by the account representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Company {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    /// Whether the company's directors have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided).
    pub directors_provided: bool,

    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,

    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,

    /// Whether the company's owners have been provided.
    ///
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that all owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    pub owners_provided: bool,

    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Whether the company's business ID number was provided.
    #[serde(default)]
    pub tax_id_provided: bool,

    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,

    /// Whether the company's business VAT number was provided.
    #[serde(default)]
    pub vat_id_provided: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferSchedule {
    /// The number of days charges for the account will be held before being paid out.
    pub delay_days: u32,

    /// How frequently funds will be paid out.
    ///
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: String,

    /// The day of the month funds will be paid out.
    ///
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,

    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    ///
    /// Only shown if `interval` is weekly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<Weekday>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    /// Unique identifier for the object.
    pub id: String,

    /// Optional information related to the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,

    /// The business type.
    ///
    /// Can be `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilities>,

    /// Whether the account can create live charges.
    #[serde(default)]
    pub charges_enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,

    /// The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter ISO currency code representing the default currency for the account.
    ///
    /// This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Whether account details have been submitted.
    ///
    /// Standard accounts cannot receive payouts before this is true.
    #[serde(default)]
    pub details_submitted: bool,

    /// The primary user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// External accounts (bank accounts and debit cards) currently attached to this account.
    // #[serde(default)]
    // pub external_accounts: List<ExternalAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Person>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Whether Stripe can send payouts to this account.
    #[serde(default)]
    pub payouts_enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountRequirements>,

    /// Account options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptance>,

    /// The Stripe account type.
    ///
    /// Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
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
    pub requested_capabilities: Option<Vec<RequestedCapability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettingsParams>,
}


#[derive(Debug, Serialize, Deserialize)]
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

/// An enum representing the possible values of an `Account`'s `business_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BusinessType {
    Company,
    Individual,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RequestedCapability {
    CardIssuing,
    CardPayments,
    LegacyPayments,
    PlatformPayments,
}

/// An enum representing the possible values of an `TransferScheduleParams`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransferScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AcceptTos {
    pub date: Timestamp,

    pub ip: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<BrandingSettingsParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CardPaymentsSettingsParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsSettingsParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<PayoutSettingsParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompanyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<Dob>,

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandingSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOnParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentsSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PayoutSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<TransferScheduleParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentParams>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeclineChargeOnParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationDocumentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<DelayDays>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<TransferScheduleInterval>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<Weekday>,
}