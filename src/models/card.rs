use super::Metadata;

#[derive(Debug, Serialize)]
pub struct CardCreateParams<'a> {
    pub number: &'a str,
    pub exp_month: u32,
    pub exp_year: u32,
    pub cvc: Option<&'a str>,
    pub currency: Option<&'a str>,
    pub address_line1: Option<&'a str>,
    pub address_line2: Option<&'a str>,
    pub address_city: Option<&'a str>,
    pub address_state: Option<&'a str>,
    pub address_zip: Option<&'a str>,
    pub address_country: Option<&'a str>,
    pub default_for_currency: Option<bool>,
    pub metadata: &'a Metadata,
    pub name: Option<&'a str>
}

#[derive(Debug, Serialize)]
pub struct CardUpdateParams<'a> {
    pub exp_month: u32,
    pub exp_year: u32,
    pub address_line1: Option<&'a str>,
    pub address_line2: Option<&'a str>,
    pub address_city: Option<&'a str>,
    pub address_state: Option<&'a str>,
    pub address_zip: Option<&'a str>,
    pub address_country: Option<&'a str>,
    pub default_for_currency: Option<bool>,
    pub metadata: &'a Metadata,
    pub name: Option<&'a str>
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: String,
    pub country: String,
    pub customer: Option<String>,
    pub cvc_check: Option<String>,
    pub default_for_currency: Option<bool>,
    pub dynamic_last4: Option<String>,
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: String,
    pub last4: String,
}
