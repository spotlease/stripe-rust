extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod stripe_models;
mod client;
mod models;
mod request;

pub use client::Client;
pub use request::Request;

#[cfg(test)]
mod tests {
    use super::{Request, Client};

    #[test]
    fn account_list_bank_accounts() {
        let client = Client {};
        let result = Request::build().list_accounts().execute(&client);
        assert!(result.is_ok());
    }
}
