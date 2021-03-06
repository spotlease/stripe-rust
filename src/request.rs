use reqwest::Method;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::marker::PhantomData;

use crate::client::Client;
use crate::models::Result as StripeResult;

#[derive(Debug)]
pub struct Request<Q: Serialize, B: Serialize, R: DeserializeOwned> {
    pub method: Method,
    pub path: String,
    pub query: Option<Q>,
    pub body: Option<B>,
    _marker: PhantomData<R>,
}

impl<Q: Serialize, B: Serialize, R: DeserializeOwned> Request<Q, B, R> {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method: method,
            path: path,
            query: None,
            body: None,
            _marker: PhantomData,
        }
    }
    pub fn new_with_query(method: Method, path: String, query: Q) -> Self {
        Self {
            method: method,
            path: path,
            query: Some(query),
            body: None,
            _marker: PhantomData,
        }
    }
    pub fn new_with_body(method: Method, path: String, body: B) -> Self {
        Self {
            method: method,
            path: path,
            query: None,
            body: Some(body),
            _marker: PhantomData,
        }
    }
    pub fn send(self, client: &Client) -> StripeResult<R> {
        client.execute(self)
    }
    pub fn send_on_behalf_of(self, client: &Client, behalf_of: &str) -> StripeResult<R> {
        let mut headers = HeaderMap::new();
        headers.insert("Stripe-Account", HeaderValue::from_str(behalf_of).unwrap()); //TODO: avoid unwrap
        let options = RequestOptions { headers };
        client.execute_with_options(self, options)
    }
}

pub struct RequestOptions {
    pub headers: HeaderMap
}

pub type SimpleRequest<R> = Request<(), (), R>;
pub type RequestWithQuery<Q, R> = Request<Q, (), R>;
pub type RequestWithBody<B, R> = Request<(), B, R>;
