// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::PayoutId;
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, BankAccount, Card, Currency};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Payout".
///
/// For more details see [https://stripe.com/docs/api/payouts/object](https://stripe.com/docs/api/payouts/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Payout {
    /// Unique identifier for the object.
    pub id: PayoutId,

    /// Amount (in %s) to be transferred to your bank account or debit card.
    pub amount: i64,

    /// Date the payout is expected to arrive in the bank.
    ///
    /// This factors in delays like weekends or bank holidays.
    pub arrival_date: Timestamp,

    /// Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,

    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of the bank account or card the payout was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Expandable<PayoutDestination>>,

    /// If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Error code explaining reason for payout failure if available.
    ///
    /// See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for payout failure if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces](/blog/instant-payouts-for-marketplaces) for more information.).
    pub method: String,

    /// The source balance this payout came from.
    ///
    /// One of `card` or `bank_account`.
    pub source_type: String,

    /// Extra information about a payout to be displayed on the user's bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Current status of the payout (`paid`, `pending`, `in_transit`, `canceled` or `failed`).
    ///
    /// A payout will be `pending` until it is submitted to the bank, at which point it becomes `in_transit`.
    /// It will then change to `paid` if the transaction goes through.
    /// If it does not go through successfully, its status will change to `failed` or `canceled`.
    pub status: String,

    /// Can be `bank_account` or `card`.
    #[serde(rename = "type")]
    pub type_: PayoutType,
}

impl Object for Payout {
    type Id = PayoutId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payout"
    }
}

/// The parameters for `Payout::create`.
#[derive(Clone, Debug, Serialize)]
pub struct PayoutCreateParams<'a> {
    /// A positive integer in cents representing how much to payout.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A set of key-value pairs that you can attach to a payout object.
    ///
    /// It can be useful for storing additional information about the payout in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces for more information](https://stripe.com/blog/instant-payouts-for-marketplaces).).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PayoutMethod>,

    /// The source balance to draw this payout from.
    ///
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the balances API.
    /// One of `bank_account` or `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PayoutSourceType>,

    /// A string to be displayed on the recipient's bank or card statement.
    ///
    /// This may be at most 22 characters.
    /// Attempting to use a `statement_descriptor` longer than 22 characters will return an error.
    /// Note: Most banks will truncate this information and/or display it inconsistently.
    /// Some may not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

/// The parameters for `Payout::list`.
#[derive(Clone, Debug, Serialize)]
pub struct PayoutListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PayoutId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<PayoutId>,

    /// Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

/// The parameters for `Payout::update`.
#[derive(Clone, Debug, Serialize)]
pub struct PayoutUpdateParams<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A set of key-value pairs that you can attach to a payout object.
    ///
    /// It can be useful for storing additional information about the payout in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum PayoutDestination {
    BankAccount(BankAccount),
    Card(Card),
}

/// An enum representing the possible values of an `PayoutCreateParams`'s `method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethod {
    Instant,
    Standard,
}

/// An enum representing the possible values of an `PayoutCreateParams`'s `source_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutSourceType {
    BankAccount,
    Card,
}

/// An enum representing the possible values of an `Payout`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
}
