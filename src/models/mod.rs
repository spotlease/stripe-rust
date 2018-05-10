mod list;
mod params;
mod account;
mod external_accounts;
mod bank_account;
mod card;
mod customer;
mod address;
mod currency;
mod discount;
mod coupon;
mod source;
mod subscription;
mod plan;
mod charge;
mod refund;
mod deleted;

pub use self::list::*;
pub use self::params::*;
pub use self::account::*;
pub use self::external_accounts::*;
pub use self::bank_account::*;
pub use self::card::*;
pub use self::customer::*;
pub use self::address::*;
pub use self::currency::*;
pub use self::discount::*;
pub use self::coupon::*;
pub use self::source::*;
pub use self::subscription::*;
pub use self::plan::*;
pub use self::charge::*;
pub use self::refund::*;
pub use self::deleted::*;

use error::Error as StripeError;

pub type Result<T> = ::std::result::Result<T, StripeError>;