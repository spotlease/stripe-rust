use resx::{ResxInstanceRB, ResxPath};
use serde::ser::Serialize;
use serde::de::Deserialize;
use reqwest::Method;

use crate::request::{Request, SimpleRequest, RequestWithBody, RequestWithQuery};
use self::account::AccountsRB;
use self::customer::CustomersRB;
use self::charge::ChargesRB;
use self::transfer::TransfersRB;
use self::refund::RefundsRB;
use crate::models::{List, Deleted};

macro_rules! resource {
    ($resource_name:ident, $builder:tt) => (
        pub fn $resource_name(mut self) -> $builder {
            self.0.push_str(concat!('/', stringify!($resource_name), 's'));
            $builder::new(self.0)
        }
    )
}

pub mod account;
pub mod customer;
pub mod charge;
pub mod transfer;
pub mod refund;

pub trait StripeResourceRB<LP, CP, M, IRB>: ResxPath
where
    LP: Serialize,
    CP: Serialize,
    for<'de> M: Deserialize<'de>,
    IRB: ResxInstanceRB,
{
    fn list_all(self) -> SimpleRequest<List<M>> {
        Request::new(Method::GET, self.path())
    }
    fn list(self, params: &LP) -> RequestWithQuery<&LP, List<M>> {
        Request::new_with_query(Method::GET, self.path(), params)
    }
    fn create(self, params: &CP) -> RequestWithBody<&CP, M> {
        Request::new_with_body(Method::POST, self.path(), params)
    }
    fn with_id<I: AsRef<str>>(self, id: I) -> IRB {
        let mut path = self.path();
        path.push('/');
        path.push_str(id.as_ref());
        IRB::new(path)
    }
}

pub trait StripeResourceInstanceRB<UP, M>: ResxPath
where
    for<'de> M: Deserialize<'de>,
    UP: Serialize,
{
    fn retrieve(self) -> SimpleRequest<M> {
        Request::new(Method::GET, self.path())
    }
    fn update(self, params: &UP) -> RequestWithBody<&UP, M> {
        Request::new_with_body(Method::POST, self.path(), params)
    }
    fn delete(self) -> SimpleRequest<Deleted> {
        Request::new(Method::DELETE, self.path())
    }
}

pub struct RootRB(String);

impl RootRB {
    pub fn new() -> RootRB {
        RootRB(String::with_capacity(256))
    }
    resource!(account, AccountsRB);
    resource!(customer, CustomersRB);
    resource!(charge, ChargesRB);
    resource!(transfer, TransfersRB);
    resource!(refund, RefundsRB);
}
