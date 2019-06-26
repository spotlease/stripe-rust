use reqwest::Method;

use crate::models::{Charge, ChargeCreateParams, ChargeUpdateParams, ChargeListParams};
use crate::request::{Request, SimpleRequest};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct ChargesRB(String);

impl<'a> super::StripeResourceRB<ChargeListParams<'a>, ChargeCreateParams<'a>, Charge, ChargeRB> for ChargesRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct ChargeRB(String);

//TODO: prevent empty charge ID, or any other ID
impl ChargeRB {
    pub fn capture<'a>(mut self) -> SimpleRequest<Charge> {
        
        self.0.push_str("/capture");
        Request::new(Method::POST, self.0)
    }
}

impl<'a> super::StripeResourceInstanceRB<ChargeUpdateParams<'a>, Charge> for ChargeRB {}