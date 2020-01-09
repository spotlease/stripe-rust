use crate::models::{Transfer, TransferCreateParams, TransferUpdateParams, TransferListParams};

use resx::{ResxInstanceRB, ResxPath, ResxRB};

#[derive(ResxPath, ResxRB)]
pub struct TransfersRB(String);

impl<'a> super::StripeResourceRB<TransferListParams<'a>, TransferCreateParams<'a>, Transfer, TransferRB> for TransfersRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct TransferRB(String);

impl<'a> super::StripeResourceInstanceRB<TransferUpdateParams<'a>, Transfer> for TransferRB {}

impl TransferRB {
    resource!(reversal, TransferReversalsRB);
}

use crate::models::{TransferReversal, TransferReversalCreateParams, TransferReversalUpdateParams, TransferReversalListParams};

#[derive(ResxPath, ResxRB)]
pub struct TransferReversalsRB(String);

impl<'a> super::StripeResourceRB<TransferReversalListParams<'a>, TransferReversalCreateParams<'a>, TransferReversal, TransferReversalRB> for TransferReversalsRB {}

#[derive(ResxPath, ResxInstanceRB)]
pub struct TransferReversalRB(String);

impl<'a> super::StripeResourceInstanceRB<TransferReversalUpdateParams<'a>, TransferReversal> for TransferReversalRB {}