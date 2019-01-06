extern crate serde_json;

use std::default::Default;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::multipart::Form as MultipartForm;
use reqwest::Client as HttpClient;
use reqwest::{Response, StatusCode};

pub const CSS_VALIDATOR_URI: &str = "https://jigsaw.w3.org/css-validator/validator";

#[derive(Serialize, Deserialize)]
pub struct CssResult {
    pub errorcount: u8,
    pub warningcount: u8,
}

#[derive(Serialize, Deserialize)]
pub struct CssValidation {
    pub uri: String,
    pub checkedby: String,
    pub csslevel: String,
    pub date: String,
    pub timestamp: String,
    pub validity: bool,
    pub result: CssResult,
    pub warnings: Option<Vec<Message>>,
    pub errors: Option<Vec<Message>>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub source: String,
    pub message: String,
    pub line: u8,
    #[serde(rename = "type")]
    pub _type: String,
    pub level: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CssValidatorResult {
    pub cssvalidation: CssValidation,
}

pub struct CssValidatorOpts {
    validator_uri: String,
}

impl CssValidation {
    pub fn is_valid(&self) -> bool {
        match self.errors {
            Some(ref err) => err.is_empty(),
            None => true,
        }
    }
}

impl CssValidatorResult {
    pub fn is_valid(&self) -> bool {
        self.cssvalidation.is_valid()
    }
}

impl Default for CssValidatorOpts {
    fn default() -> CssValidatorOpts {
        CssValidatorOpts {
            validator_uri: CSS_VALIDATOR_URI.to_string(),
        }
    }
}

pub struct CssValidator {
    pub validator_uri: String,
    http_client: HttpClient,
}

/// # Examples
///
/// ```
/// use std::default::Default;
/// use w3c_validators::CssValidator;
/// let nu = CssValidator::new(Default::default());
/// match nu.validate_text("tbody th{width: /* 25%} */") {
///     Some(result) => {
///         if !result.is_valid() {
///             for msg in result.cssvalidation.errors.unwrap() {
///                 println!("{}: {}", msg._type, msg.message);
///             }
///         }
///     }
///     None => {},
/// }
/// ```
impl CssValidator {
    pub fn new(opts: CssValidatorOpts) -> CssValidator {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("HttpClient failed to construct");
        CssValidator {
            validator_uri: opts.validator_uri,
            http_client,
        }
    }

    fn fetch_result(&self, mut response: Response) -> Option<CssValidatorResult> {
        match response.text() {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn validate_uri(&self, uri: &str) -> Option<CssValidatorResult> {
        let url = &format!("{}?uri={}&output=json", CSS_VALIDATOR_URI, uri);
        match self
            .http_client
            .get(url)
            .header(USER_AGENT, ua_validator!())
            .send()
        {
            Err(_) => None,
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::NOT_MODIFIED => self.fetch_result(response),
                _ => None,
            },
        }
    }

    pub fn validate_text(&self, text: &'static str) -> Option<CssValidatorResult> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(ua_validator!()));
        let form = MultipartForm::new()
            .text("text", text)
            .text("output", "json");
        match self
            .http_client
            .post(CSS_VALIDATOR_URI)
            .multipart(form)
            .headers(headers)
            .send()
        {
            Err(_) => None,
            Ok(response) => match response.status() {
                StatusCode::OK | StatusCode::NOT_MODIFIED => self.fetch_result(response),
                _ => None,
            },
        }
    }
}
