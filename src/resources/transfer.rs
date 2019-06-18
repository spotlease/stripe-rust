// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ChargeId, TransferId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Account, BalanceTransaction, Charge, Currency, TransferReversal};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Transfer".
///
/// For more details see [https://stripe.com/docs/api/transfers/object](https://stripe.com/docs/api/transfers/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transfer {
    /// Unique identifier for the object.
    pub id: TransferId,

    /// Amount in %s to be transferred.
    pub amount: i64,

    /// Amount in %s reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,

    /// Balance transaction that describes the impact of this transfer on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time that this record of the transfer was first created.
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

    /// ID of the Stripe account the transfer was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Expandable<Account>>,

    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<Expandable<Charge>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A set of key-value pairs that you can attach to a transfer object.
    ///
    /// It can be useful for storing additional information about the transfer in a structured format.
    pub metadata: Metadata,

    /// A list of reversals that have been applied to the transfer.
    pub reversals: List<TransferReversal>,

    /// Whether the transfer has been fully reversed.
    ///
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,

    /// ID of the charge or payment that was used to fund the transfer.
    ///
    /// If null, the transfer was funded from the available balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<Expandable<Charge>>,

    /// The source balance this transfer came from.
    ///
    /// One of `card` or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<TransferSourceType>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#grouping-transactions) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl Object for Transfer {
    type Id = TransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer"
    }
}

/// The parameters for `Transfer::create`.
#[derive(Clone, Debug, Serialize)]
pub struct TransferCreateParams<'a> {
    /// A positive integer in %s representing how much to transfer.
    pub amount: i64,

    /// 3-letter [ISO code for currency](https://stripe.com/docs/payouts).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// You can use this parameter to transfer funds from a charge before they are added to your available balance.
    ///
    /// A pending balance will transfer immediately but the funds will not become available until the original charge becomes available.
    /// [See the Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-availability) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<ChargeId>,

    /// The source balance to use for this transfer.
    ///
    /// One of `bank_account` or `card`.
    /// For most users, this will default to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<TransferSourceType>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#grouping-transactions) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The parameters for `Transfer::list`.
#[derive(Clone, Debug, Serialize)]
pub struct TransferListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TransferId>,

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
    pub starting_after: Option<TransferId>,

    /// Only return transfers with the specified transfer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The parameters for `Transfer::update`.
#[derive(Clone, Debug, Serialize)]
pub struct TransferUpdateParams<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// An enum representing the possible values of an `Transfer`'s `source_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransferSourceType {
    BankAccount,
    Card,
}
