// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{CustomerId, TokenId};
use crate::params::{Expand, Metadata, Object, Timestamp};
use crate::resources::{BankAccount, Card};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Token".
///
/// For more details see [https://stripe.com/docs/api/tokens/object](https://stripe.com/docs/api/tokens/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// Unique identifier for the object.
    pub id: TokenId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,

    /// IP address of the client that generated the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: TokenType,

    /// Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}

impl Object for Token {
    type Id = TokenId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "token"
    }
}

/// The parameters for `Token::create`.
#[derive(Clone, Debug, Serialize)]
pub struct TokenCreateParams<'a> {
    /// Information for the account this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<TokenCreateParamsAccount>,

    /// The customer (owned by the application's account) for which to create a token.
    ///
    /// For use only with [Stripe Connect](https://stripe.com/docs/connect).
    /// Also, this can be used only with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// For more details, see [Shared Customers](https://stripe.com/docs/connect/shared-customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The PII this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<TokenCreateParamsPii>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<TokenCreateParamsAccountCompany>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<TokenCreateParamsAccountIndividual>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsPii {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountCompany {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TokenCreateParamsAccountCompanyAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<TokenCreateParamsAccountCompanyAddressKana>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<TokenCreateParamsAccountCompanyAddressKanji>,

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
pub struct TokenCreateParamsAccountIndividual {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TokenCreateParamsAccountIndividualAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<TokenCreateParamsAccountIndividualAddressKana>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<TokenCreateParamsAccountIndividualAddressKanji>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<TokenCreateParamsAccountIndividualDob>,

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
    pub verification: Option<TokenCreateParamsAccountIndividualVerification>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountCompanyAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountCompanyAddressKana {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountCompanyAddressKanji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountIndividualAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountIndividualAddressKana {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountIndividualAddressKanji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountIndividualDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountIndividualVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<TokenCreateParamsAccountIndividualVerificationDocument>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCreateParamsAccountIndividualVerificationDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

/// An enum representing the possible values of an `Token`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Account,
    BankAccount,
    Card,
    Pii,
}
