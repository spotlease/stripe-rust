use std::marker::PhantomData;
use serde::de::DeserializeOwned;
use serde_json;

use models;
use stripe_models::{List, StripeResult};
use client::Client;

pub struct Request;

impl Request {
    pub fn build() -> Self {
        Request {}
    }
    pub fn list_accounts(self) -> ExecutableRequest<models::Account> {
        ExecutableRequest::new()
    }
    // pub fn for_account(self) {

    // }
}

// struct AccountRequest;

// impl AccountRequest {
//     pub fn retrive(self) {

//     }
// }

pub struct ExecutableRequest<T: DeserializeOwned> {
    _marker: PhantomData<T>,
}

impl<T: DeserializeOwned> ExecutableRequest<T> {
    pub fn new() -> Self {
        ExecutableRequest {
            _marker: PhantomData
        }
    }

    pub fn execute(self, client: &Client) -> StripeResult<T> {
        let json = r#"{
                        "data":"asd"
                      }"#;
        let result = serde_json::from_str(json).unwrap();
        Ok(result)
    }
}