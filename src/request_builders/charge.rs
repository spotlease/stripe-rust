use models::{Charge, ChargeCreateParams, ChargeUpdateParams, ChargeListParams};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct ChargesRB(String);

impl<'a> super::StripeResourceRB<ChargeListParams<'a>, ChargeCreateParams<'a>, Charge, ChargeRB> for ChargesRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct ChargeRB(String);

impl<'a> super::StripeResourceInstanceRB<ChargeUpdateParams<'a>, Charge> for ChargeRB {}