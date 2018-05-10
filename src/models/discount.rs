use models::{Timestamp, Coupon};

/// The resource representing a Stripe discount.
///
/// For more details see https://stripe.com/docs/api#discounts.
#[derive(Debug, Deserialize)]
pub struct Discount {
    pub coupon: Coupon,
    pub customer: String,
    pub subscription: Option<String>,

    pub start: Timestamp,
    pub end: Option<Timestamp>,
}
