use super::{Metadata, Timestamp, Currency, Identifier, IdentifierParam};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureReason {
    LostOrStolenCard,
    ExpiredOrCanceledCard,
    Unknown
}
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
    Canceled
}

#[derive(Debug, Default, Serialize)]
pub struct RefundCreateParams<'a> {
    pub charge: IdentifierParam<'a>,
    pub amount: u64,
    pub metadata: Option<&'a Metadata>,
    pub reason: Option<RefundReason>,
    pub refund_application_fee: Option<bool>,
    pub reverse_transfer: Option<bool>
}

#[derive(Debug, Default, Serialize)]
pub struct RefundUpdateParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a Metadata>
}

#[derive(Debug, Default, Serialize)]
pub struct RefundListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<IdentifierParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<IdentifierParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<IdentifierParam<'a>>
}

/// The resource representing a Stripe refund.
///
/// For more details see https://stripe.com/docs/api#refunds.
#[derive(Debug, Deserialize)]
pub struct Refund {
    pub id: Identifier,
    pub amount: u64,
    pub balance_transaction: Identifier,
    pub charge: Identifier,
    pub created: Timestamp,
    pub currency: Currency,
    pub failure_balance_transaction: Option<Identifier>,
    pub failure_reason: Option<FailureReason>,
    pub metadata: Metadata,
    pub reason: Option<RefundReason>,
    pub receipt_number: Option<String>,
    pub status: RefundStatus
}