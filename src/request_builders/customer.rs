use serde::Serialize;
use reqwest::Method;

use crate::models::{Customer, CustomerParams, CustomerListParams, Source, Deleted};
use crate::request::{Request, RequestWithBody, SimpleRequest};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct CustomersRB(String);

impl<'a> super::StripeResourceRB<CustomerListParams<'a>, CustomerParams<'a>, Customer, CustomerRB> for CustomersRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct CustomerRB(String);

#[derive(Debug, Serialize)]
pub struct AttachSourceBody<'a> {
    source: &'a str
}

impl CustomerRB {
    pub fn attach_source<'a>(mut self, source_id: &str) -> RequestWithBody<AttachSourceBody, Source> {
        
        self.0.push_str("/sources");
        Request::new_with_body(Method::POST, self.0, AttachSourceBody {
            source: source_id
        })
    }
    pub fn detach_source<'a>(mut self, source_id: &str) -> SimpleRequest<Deleted> {
        self.0.push_str("/sources/");
        self.0.push_str(source_id);
        Request::new(Method::DELETE, self.0)
    }
}

impl<'a> super::StripeResourceInstanceRB<CustomerParams<'a>, Customer> for CustomerRB {}