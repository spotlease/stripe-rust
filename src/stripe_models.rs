pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

#[derive(Debug)]
pub struct StripeError;


pub type StripeResult<T> = Result<T, StripeError>;