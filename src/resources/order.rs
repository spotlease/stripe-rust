// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{CouponId, CustomerId, OrderId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Charge, Currency, Customer, OrderItem, OrderReturn, Shipping};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Order".
///
/// For more details see [https://stripe.com/docs/api/orders/object](https://stripe.com/docs/api/orders/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    /// Unique identifier for the object.
    pub id: OrderId,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_returned: Option<i64>,

    /// ID of the Connect Application that created the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i64>,

    /// The ID of the payment used to pay for the order.
    ///
    /// Present if the order status is `paid`, `fulfilled`, or `refunded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Expandable<Charge>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The customer used for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_coupon_code: Option<String>,

    /// List of items constituting the order.
    ///
    /// An order can have up to 25 items.
    pub items: Vec<OrderItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    #[serde(default)]
    pub returns: List<OrderReturn>,

    /// The shipping method that is currently selected for this order, if any.
    ///
    /// If present, it is equal to one of the `id`s of shipping methods in the `shipping_methods` array.
    /// At order creation time, if there are multiple shipping methods, Stripe will automatically selected the first method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_shipping_method: Option<String>,

    /// The shipping address for the order.
    ///
    /// Present if the order is for goods to be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// A list of supported shipping methods for this order.
    ///
    /// The desired shipping method can be specified either by updating the order, or when paying it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_methods: Option<Vec<ShippingMethod>>,

    /// Current order status.
    ///
    /// One of `created`, `paid`, `canceled`, `fulfilled`, or `returned`.
    /// More details in the [Orders Guide](https://stripe.com/docs/orders/guide#understanding-order-statuses).
    pub status: OrderStatus,

    /// The timestamps at which the order status was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<StatusTransitions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,

    /// The user's order ID if it is different from the Stripe order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_id: Option<String>,
}

impl Object for Order {
    type Id = OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingMethod {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the line item.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The estimated delivery date for the given shipping method.
    ///
    /// Can be either a specific date or a range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<DeliveryEstimate>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,

    /// Unique identifier for the object.
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeliveryEstimate {
    /// If `type` is `"exact"`, `date` will be the expected delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// If `type` is `"range"`, `earliest` will be be the earliest delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<String>,

    /// If `type` is `"range"`, `latest` will be the latest delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,

    /// The type of estimate.
    ///
    /// Must be either `"range"` or `"exact"`.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfiled: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<Timestamp>,
}

/// The parameters for `Order::create`.
#[derive(Clone, Debug, Serialize)]
pub struct OrderCreateParams<'a> {
    /// A coupon code that represents a discount to be applied to this order.
    ///
    /// Must be one-time duration and in same currency as the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of an existing customer to use for this order.
    ///
    /// If provided, the customer email and shipping address will be used to create the order.
    /// Subsequently, the customer will also be charged to pay the order.
    /// If `email` or `shipping` are also provided, they will override the values retrieved from the customer object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// List of items constituting the order.
    ///
    /// An order can have up to 25 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderCreateParamsItems>>,

    /// A set of key-value pairs that you can attach to an order object.
    ///
    /// Limited to 500 characters.
    /// Metadata can be useful for storing additional information about the order in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Shipping address for the order.
    ///
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderCreateParamsShipping>,
}

/// The parameters for `Order::list`.
#[derive(Clone, Debug, Serialize)]
pub struct OrderListParams<'a> {
    /// Date this order was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return orders for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<OrderId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return orders with the given IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

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
    pub starting_after: Option<OrderId>,

    /// Only return orders that have the given status.
    ///
    /// One of `created`, `paid`, `fulfilled`, or `refunded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatusFilter>,

    /// Filter orders based on when they were paid, fulfilled, canceled, or returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<OrderListParamsStatusTransitions>,

    /// Only return orders with the given upstream order IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_ids: Option<Vec<String>>,
}

/// The parameters for `Order::update`.
#[derive(Clone, Debug, Serialize)]
pub struct OrderUpdateParams<'a> {
    /// A coupon code that represents a discount to be applied to this order.
    ///
    /// Must be one-time duration and in same currency as the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A set of key-value pairs that you can attach to a product object.
    ///
    /// It can be useful for storing additional information about the order in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The shipping method to select for fulfilling this order.
    ///
    /// If specified, must be one of the `id`s of a shipping method in the `shipping_methods` array.
    /// If specified, will overwrite the existing selected shipping method, updating `items` as necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_shipping_method: Option<&'a str>,

    /// Tracking information once the order has been fulfilled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderUpdateParamsShipping>,

    /// Current order status.
    ///
    /// One of `created`, `paid`, `canceled`, `fulfilled`, or `returned`.
    /// More detail in the [Orders Guide](https://stripe.com/docs/orders/guide#understanding-order-statuses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderCreateParamsItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<OrderCreateParamsItemsType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderCreateParamsShipping {
    pub address: OrderCreateParamsShippingAddress,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderListParamsStatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfilled: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<RangeQuery<Timestamp>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderUpdateParamsShipping {
    pub carrier: String,

    pub tracking_number: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderCreateParamsShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    pub line1: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// An enum representing the possible values of an `OrderCreateParamsItems`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderCreateParamsItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

/// An enum representing the possible values of an `Order`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Canceled,
    Created,
    Fulfilled,
    Paid,
    Returned,
}

/// An enum representing the possible values of an `OrderListParams`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatusFilter {
    Created,
    Fulfilled,
    Paid,
    Refunded,
}
