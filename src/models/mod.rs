mod account;
mod bank_account;
mod card;
mod charge;
mod currency;
mod customer;
mod external_accounts;
mod params;
mod person;
mod refund;
mod source;
mod transfer;
mod types;

pub use self::account::*;
pub use self::bank_account::*;
pub use self::card::*;
pub use self::charge::*;
pub use self::currency::*;
pub use self::customer::*;
pub use self::external_accounts::*;
pub use self::params::*;
pub use self::person::*;
pub use self::refund::*;
pub use self::source::*;
pub use self::transfer::*;
pub use self::types::*;

use crate::error::Error as StripeError;

pub type Result<T> = ::std::result::Result<T, StripeError>;
