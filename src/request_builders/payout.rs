use reqwest::Method;

use crate::models::{Payout, PayoutCreateParams, PayoutUpdateParams, PayoutListParams};
use crate::request::{Request, SimpleRequest};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct PayoutsRB(String);

impl<'a> super::StripeResourceRB<PayoutListParams<'a>, PayoutCreateParams<'a>, Payout, PayoutRB> for PayoutsRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct PayoutRB(String);

impl PayoutRB {
    pub fn capture<'a>(mut self) -> SimpleRequest<Payout> {
        
        self.0.push_str("/capture");
        Request::new(Method::POST, self.0)
    }
}

impl super::StripeResourceInstanceRB<PayoutUpdateParams, Payout> for PayoutRB {}