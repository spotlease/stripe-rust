// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::SkuId;
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{Currency, PackageDimensions, Product};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SKU".
///
/// For more details see [https://stripe.com/docs/api/skus/object](https://stripe.com/docs/api/skus/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sku {
    /// Unique identifier for the object.
    pub id: SkuId,

    /// Whether the SKU is available for purchase.
    #[serde(default)]
    pub active: bool,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Metadata>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Inventory>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,

    /// The ID of the product this SKU is associated with.
    ///
    /// The product must be currently active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Expandable<Product>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,
}

impl Object for Sku {
    type Id = SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "sku"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inventory {
    /// The count of inventory available.
    ///
    /// Will be present if and only if `type` is `finite`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Inventory type.
    ///
    /// Possible values are `finite`, `bucket` (not quantified), and `infinite`.
    #[serde(rename = "type")]
    pub type_: String,

    /// An indicator of the inventory available.
    ///
    /// Possible values are `in_stock`, `limited`, and `out_of_stock`.
    /// Will be present if and only if `type` is `bucket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The parameters for `Sku::create`.
#[derive(Clone, Debug, Serialize)]
pub struct SkuCreateParams<'a> {
    /// Whether the SKU is available for purchase.
    ///
    /// Default to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// If, for example, a product's attributes are `["size", "gender"]`, a valid SKU has the following dictionary of attributes: `{"size": "Medium", "gender": "Unisex"}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SkuCreateParamsAttributes>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The identifier for the SKU.
    ///
    /// Must be unique.
    /// If not provided, an identifier will be randomly generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,

    /// Description of the SKU's inventory.
    pub inventory: SkuCreateParamsInventory,

    /// A set of key-value pairs that you can attach to a SKU object.
    ///
    /// It can be useful for storing additional information about the SKU in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<SkuCreateParamsPackageDimensions>,

    /// The cost of the item as a nonnegative integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    pub price: i64,
}

/// The parameters for `Sku::list`.
#[derive(Clone, Debug, Serialize)]
pub struct SkuListParams<'a> {
    /// Only return SKUs that are active or inactive (e.g., pass `false` to list all inactive products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Only return SKUs that have the specified key-value pairs in this partially constructed dictionary.
    ///
    /// Can be specified only if `product` is also supplied.
    /// For instance, if the associated product has attributes `["color", "size"]`, passing in `attributes[color]=red` returns all the SKUs for this product that have `color` set to `red`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SkuListParamsAttributes>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SkuId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return SKUs with the given IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

    /// Only return SKUs that are either in stock or out of stock (e.g., pass `false` to list all SKUs that are out of stock).
    ///
    /// If no value is provided, all SKUs are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,

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
    pub starting_after: Option<SkuId>,
}

/// The parameters for `Sku::update`.
#[derive(Clone, Debug, Serialize)]
pub struct SkuUpdateParams<'a> {
    /// Whether this SKU is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A dictionary of attributes and values for the attributes defined by the product.
    ///
    /// When specified, `attributes` will partially update the existing attributes dictionary on the product, with the postcondition that a value must be present for each attribute key on the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SkuUpdateParamsAttributes>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL of an image for this SKU, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,

    /// Description of the SKU's inventory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<SkuUpdateParamsInventory>,

    /// A set of key-value pairs that you can attach to a SKU object.
    ///
    /// It can be useful for storing additional information about the SKU in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The dimensions of this SKU for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<SkuUpdateParamsPackageDimensions>,

    /// The cost of the item as a positive integer in the smallest currency unit (that is, 100 cents to charge $1.00, or 100 to charge ¥100, Japanese Yen being a zero-decimal currency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkuCreateParamsInventory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<SkuCreateParamsInventoryType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<SkuCreateParamsInventoryValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkuCreateParamsPackageDimensions {
    pub height: f64,

    pub length: f64,

    pub weight: f64,

    pub width: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkuUpdateParamsInventory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<SkuUpdateParamsInventoryType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<SkuUpdateParamsInventoryValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkuUpdateParamsPackageDimensions {
    pub height: f64,

    pub length: f64,

    pub weight: f64,

    pub width: f64,
}

/// An enum representing the possible values of an `SkuCreateParamsInventory`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkuCreateParamsInventoryType {
    Bucket,
    Finite,
    Infinite,
}

/// An enum representing the possible values of an `SkuCreateParamsInventory`'s `value` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkuCreateParamsInventoryValue {
    InStock,
    Limited,
    OutOfStock,
}

/// An enum representing the possible values of an `SkuUpdateParamsInventory`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkuUpdateParamsInventoryType {
    Bucket,
    Finite,
    Infinite,
}

/// An enum representing the possible values of an `SkuUpdateParamsInventory`'s `value` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkuUpdateParamsInventoryValue {
    InStock,
    Limited,
    OutOfStock,
}
