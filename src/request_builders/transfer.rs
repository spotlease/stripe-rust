use models::{Transfer, TransferCreateParams, TransferUpdateParams, TransferListParams};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct TransfersRB(String);

impl<'a> super::StripeResourceRB<TransferListParams<'a>, TransferCreateParams<'a>, Transfer, TransferRB> for TransfersRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct TransferRB(String);

impl<'a> super::StripeResourceInstanceRB<TransferUpdateParams<'a>, Transfer> for TransferRB {}