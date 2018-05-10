use super::Metadata;
use super::CardCreateParams;
use super::CardUpdateParams;
use super::Card;
use super::BankAccount;
use super::BankAccountCreateParams;
use super::BankAccountUpdateParams;

#[derive(Debug, Serialize)]
pub struct ExternalAccountCreateParams<'a> {
    pub external_account: ExternalAccountParam<'a>,
    pub default_for_currency: Option<bool>,
    pub metadata: Option<Metadata>
}

#[derive(Debug)]
pub enum ExternalAccountParam<'a> {
    Token(&'a str),
    // #[serde(serialize_with = "serialize_token")]
    CardDetails(CardCreateParams<'a>),    
    // #[serde(serialize_with = "serialize_token")]
    BankAccountDetails(BankAccountCreateParams<'a>)
}
use serde::Serialize;
use serde::Serializer;

impl<'a> Serialize for ExternalAccountParam<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ExternalAccountParam::Token(ref token) => {
                token.serialize(serializer)
            }
            ExternalAccountParam::CardDetails(ref card) => {
                serializer.serialize_newtype_variant("ExternalAccountParam", 1, "CardDetails", card)
                // let mut tv = serializer.serialize_tuple_variant("E", 0, "T", 1)?;
                // card.serialize(serializer);
                // serializer.serialize_field("type", "card")
            }
            ExternalAccountParam::BankAccountDetails(ref bank_account) => {
                serializer.serialize_newtype_variant("ExternalAccountParam", 1, "BankAccountDetails", bank_account)
            }
        }
    }
}

// use serde::Serializer;
// fn serialize_token<S>(card: &CardCreateParams, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
//     use serde::Serialize;
//     use serde::ser::SerializeMap;
//     card.serialize(serializer)?;
//     serializer.serialize_entry("type", "card")?
// }


#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ExternalAccountUpdateParams<'a> {
    Card(CardUpdateParams<'a>),
    BankAccount(BankAccountUpdateParams<'a>)
}

#[derive(Debug, Default, Serialize)]
pub struct ExternalAccountListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(rename = "object")]
    #[serde(skip_serializing_if = "ExternalAccountListTypeParam::is_all")]
    pub external_account_type: ExternalAccountListTypeParam
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "object")]
pub enum ExternalAccountListTypeParam {
    All,
    Card,
    BankAccount,
}

impl ExternalAccountListTypeParam {
    fn is_all(&self) -> bool {
        match self {
            &ExternalAccountListTypeParam::All => true,
            _ => false
        }
    }
}
impl Default for ExternalAccountListTypeParam {
    fn default() -> ExternalAccountListTypeParam { ExternalAccountListTypeParam::All }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "object")]
pub enum ExternalAccount {
    Card(Card),
    BankAccount(BankAccount),
}