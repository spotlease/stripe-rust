use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::marker::PhantomData;

use client::Client;
use models::Result as StripeResult;

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
}

pub type SimpleRequest<R> = Request<(), (), R>;
pub type RequestWithQuery<Q, R> = Request<Q, (), R>;
pub type RequestWithBody<B, R> = Request<(), B, R>;
