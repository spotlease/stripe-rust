use models::{Metadata, List, RangeQuery, Timestamp, Currency};

/// The set of parameters that can be used when creating a transfer.
///
/// For more details see https://stripe.com/docs/api#create_transfer
#[derive(Default, Serialize)]
pub struct TransferCreateParams<'a> {
    pub amount: u64,
    pub currency: Currency,
    pub destination: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The set of parameters that can be used when updating a transfer.
///
/// For more details see https://stripe.com/docs/api#create_transfer
#[derive(Default, Serialize)]
pub struct TransferUpdateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a Metadata>
}

/// The set of parameters that can be used when listing transfers.
///
/// For more details see https://stripe.com/docs/api#list_transfers
#[derive(Default, Serialize)]
pub struct TransferListParams<'a> {
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
    pub transfer_group: Option<&'a str>,
}

/// The resource representing a Stripe transfer reversal.
///
/// For more details see https://stripe.com/docs/api#transfer_reversal_object.
#[derive(Debug, Deserialize)]
pub struct TransferReversal {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub metadata: Metadata,
    pub transfer: String,
}

/// The resource representing a Stripe transfer.
///
/// For more details see https://stripe.com/docs/api#transfer_object.
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub amount_reversed: u64,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: Option<String>,
    pub destination: String,
    pub destination_payment: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub reversals: List<TransferReversal>,
    pub reversed: bool,
    pub source_transaction: Option<String>,
    pub source_type: String,
    pub transfer_group: String,
}
