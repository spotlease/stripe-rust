use models::{Charge, ChargeCreateParams, ChargeUpdateParams, ChargeListParams};
use request::{Request, SimpleRequest};
use reqwest::Method;

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct ChargesRB(String);

impl<'a> super::StripeResourceRB<ChargeListParams<'a>, ChargeCreateParams<'a>, Charge, ChargeRB> for ChargesRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct ChargeRB(String);

impl ChargeRB {
    pub fn capture<'a>(mut self) -> SimpleRequest<Charge> {
        
        self.0.push_str("/capture");
        Request::new(Method::Post, self.0)
    }
}

impl<'a> super::StripeResourceInstanceRB<ChargeUpdateParams<'a>, Charge> for ChargeRB {}