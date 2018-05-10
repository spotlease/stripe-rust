use models::{Account, AccountCreateParams, AccountListParams, AccountUpdateParams};
use request::{Request, SimpleRequest};
use reqwest::Method;
use models::{ExternalAccount, ExternalAccountListParams, ExternalAccountCreateParams, ExternalAccountUpdateParams};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct AccountsRB(String);

impl<'a> super::StripeResourceRB<AccountListParams<'a>, AccountCreateParams<'a>, Account, AccountRB> for AccountsRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct AccountRB(String);

impl AccountRB {
    pub fn reject(mut self) -> SimpleRequest<Account> {
        self.0.push_str("/reject");
        Request::new(Method::Post, self.0)
    }
    pub fn create_login_link(mut self) -> SimpleRequest<Account> {
        self.0.push_str("/login_links");
        Request::new(Method::Post, self.0)
    }
    resource!(external_account, ExternalAccountsRB);
}

impl<'a> super::StripeResourceInstanceRB<AccountUpdateParams<'a>, Account> for AccountRB {}

#[derive(ResxPath, ResxRB)]
pub struct ExternalAccountsRB(String);

impl<'a> super::StripeResourceRB<ExternalAccountListParams<'a>, ExternalAccountCreateParams<'a>, ExternalAccount, ExternalAccountRB> for ExternalAccountsRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct ExternalAccountRB(String);

impl<'a> super::StripeResourceInstanceRB<ExternalAccountUpdateParams<'a>, ExternalAccount> for ExternalAccountRB {}