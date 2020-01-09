#[macro_use] extern crate log;
#[macro_use] extern crate resx_derives;

mod client;
mod models;
mod request;
mod error;
mod request_builders;

pub use client::Client;
pub use request::Request;
pub use models::*;
pub use request_builders::*;
pub use error::Error;
pub use error::RequestError;

use request_builders::RootRB;

pub fn request() -> RootRB {
    RootRB::new()
}

#[cfg(test)]
mod tests {
    use super::Client;
    use super::request_builders::StripeResourceRB;

    #[test]
    fn list_accounts() {
        let secret_key = "sk_test_b7OIZOj9iD00hccDQcHakkmk";
        let client = Client::builder(secret_key).build().unwrap();
        let result = super::request().account().list_all()
            .send(&client);
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    // #[test]
    // fn account_list_bank_accounts() {

    //     let secret_key = "sk_test_b7OIZOj9iD00hccDQcHakkmk";
    //     let client = Client::builder(secret_key).build().unwrap();
    //     let result = RootRB::new().account().list()
    //         .send(&client);
    //     println!("{:?}", result);
    //     assert!(result.is_ok());
    // }
}
