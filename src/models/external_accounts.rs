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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ExternalAccountParam<'a> {
    Token(&'a str),
    Details(ExternalAccountDetailParams<'a>)
}

//TODO: support serialization
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "object")]
pub enum ExternalAccountDetailParams<'a> {
    Card(CardCreateParams<'a>),    
    BankAccount(BankAccountCreateParams<'a>)
}

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