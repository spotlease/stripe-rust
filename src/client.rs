use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Url};
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use std::time::Duration;
use models::Result as StripeResult;
use error::Error as StripeError;
use error::RequestError as StripeRequestError;
use request::Request;
use reqwest;

const DEFAULT_API_URL: &'static str = "https://api.stripe.com/v1/";

pub struct ClientBuilder {
    timeout: Option<Duration>,
    base_url: String,
    inner: ReqwestClientBuilder,
}

impl ClientBuilder {
    pub fn new<IntoString: Into<String>>(secret_key: IntoString) -> Self {
        use reqwest::header::{Authorization, Basic, ContentType, Headers};

        let mut default_headers = Headers::new();
        default_headers.set(Authorization(Basic {
            username: secret_key.into(),
            password: None,
        }));
        default_headers.set(ContentType::form_url_encoded());

        let mut reqwest_client_builder = ReqwestClient::builder();
        reqwest_client_builder.default_headers(default_headers);

        Self {
            inner: reqwest_client_builder,
            timeout: Some(Duration::from_secs(30)),
            base_url: DEFAULT_API_URL.into(),
        }
    }
    pub fn set_timeout<Timeout: Into<Option<Duration>>>(
        &mut self,
        timeout: Timeout,
    ) -> &mut ClientBuilder {
        self.timeout = timeout.into();
        self
    }
    pub fn build(mut self) -> StripeResult<Client> {
        Ok(Client {
            base_url: Url::parse(&self.base_url)?,
            inner: self.inner.build()?,
        })
    }
}

pub struct Client {
    inner: ReqwestClient,
    base_url: Url,
}

impl Client {
    pub fn builder<IntoString: Into<String>>(secret_key: IntoString) -> ClientBuilder {
        ClientBuilder::new(secret_key)
    }
    pub fn execute<Q: Serialize, B: Serialize, R: DeserializeOwned>(&self, request: Request<Q, B, R>) -> StripeResult<R> {

        let mut reqwest_request_builder = self.inner.request(request.method, self.base_url.join(&request.path[1..])?);

        if let Some(ref request_query) = request.query {
            reqwest_request_builder.query(request_query);
        }

        if let Some(ref request_body) = request.body {
            reqwest_request_builder.form(request_body);
        }

        let reqwest_request = reqwest_request_builder.build()?;

        println!("stripe request: {:?}", reqwest_request);

        {
            let request_body = reqwest_request.body();
            println!("stripe request body: {:?}", request_body);
        }

        let response = self.inner.execute(reqwest_request)?;

        #[derive(Debug, Deserialize)]
        struct ErrorWrapper {
            error: StripeRequestError
        }
        let http_status = response.status().as_u16();
        // println!("start stripe response body");
        // serde_json::from_reader(reader)
        // println!("end stripe response body");
        match http_status {
            200 => parse_response(response),
            _ => {
                let ErrorWrapper { mut error } = parse_response(response)?;
                error.http_status = http_status;
                Err(StripeError::from(error))
            }
        }
    }
}

#[cfg(debug_assertions)]
fn parse_response<T: DeserializeOwned>(mut response: reqwest::Response) -> StripeResult<T> {
    {
        let res_text = response.text()?;
        println!("stripe response body: {}", res_text);
        serde_json::from_str(&res_text)
    }
    .map_err(|err| StripeError::from(err))
}

#[cfg(not(debug_assertions))]
fn parse_response<T: DeserializeOwned>(response: reqwest::Response) -> StripeResult<T> {
    serde_json::from_reader(response).map_err(|err| StripeError::from(err))
}