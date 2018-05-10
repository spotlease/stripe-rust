use models::{Customer, CustomerParams, CustomerListParams, Source};
use request::{Request, RequestWithBody, SimpleRequest};
use reqwest::Method;

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
        Request::new_with_body(Method::Post, self.0, AttachSourceBody {
            source: source_id
        })
    }
    pub fn detach_source<'a>(mut self, source_id: &str) -> SimpleRequest<Source> {
        self.0.push_str("/sources/");
        self.0.push_str(source_id);
        Request::new(Method::Delete, self.0)
    }
}

impl<'a> super::StripeResourceInstanceRB<CustomerParams<'a>, Customer> for CustomerRB {}