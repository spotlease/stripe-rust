use models::{Refund, RefundCreateParams, RefundUpdateParams, RefundListParams};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct RefundsRB(String);

impl<'a> super::StripeResourceRB<RefundListParams<'a>, RefundCreateParams<'a>, Refund, RefundRB> for RefundsRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct RefundRB(String);

impl<'a> super::StripeResourceInstanceRB<RefundUpdateParams<'a>, Refund> for RefundRB {}