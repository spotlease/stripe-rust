mod params;
mod account;
mod external_accounts;
mod bank_account;
mod card;
mod customer;
mod address;
mod currency;
mod source;
mod charge;
mod refund;
mod transfer;

pub use self::params::*;
pub use self::account::*;
pub use self::external_accounts::*;
pub use self::bank_account::*;
pub use self::card::*;
pub use self::customer::*;
pub use self::address::*;
pub use self::currency::*;
pub use self::source::*;
pub use self::charge::*;
pub use self::refund::*;
pub use self::transfer::*;

use crate::error::Error as StripeError;

pub type Result<T> = ::std::result::Result<T, StripeError>;