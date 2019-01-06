extern crate serde_json;

use std::default::Default;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Client as HttpClient;
use reqwest::{Response, StatusCode};

pub const MARKUP_VALIDATOR_URI: &str = "https://validator.w3.org/nu/";
pub const TEXT_HTML_UTF_8: &str = "text/html; charset=utf-8";

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "lastLine")]
    pub last_line: Option<u8>,
    #[serde(rename = "lastColumn")]
    pub last_column: Option<u8>,
    #[serde(rename = "firstColumn")]
    pub first_column: Option<u8>,
    #[serde(rename = "subType")]
    pub sub_type: Option<String>,
    pub message: String,
    pub extract: Option<String>,
    #[serde(rename = "hiliteStart")]
    pub hilite_start: Option<u8>,
    #[serde(rename = "hiliteLength")]
    pub hilite_length: Option<u8>,
}

impl Message {
    pub fn is_warning(&self) -> bool {
        &self._type == "warning"
    }

    pub fn is_error(&self) -> bool {
        &self._type == "error"
    }
}

#[derive(Serialize, Deserialize)]
pub struct NuValidatorResult {
    pub url: Option<String>,
    pub messages: Vec<Message>,
    pub language: Option<String>,
}

impl NuValidatorResult {
    pub fn is_valid(&self) -> bool {
        !self.messages.iter().any(|msg| msg.is_error())
    }
}

pub struct NuValidatorOpts {
    validator_uri: String,
}

impl Default for NuValidatorOpts {
    fn default() -> NuValidatorOpts {
        NuValidatorOpts {
            validator_uri: MARKUP_VALIDATOR_URI.to_string(),
        }
    }
}

pub struct NuValidator {
    pub validator_uri: String,
    http_client: HttpClient,
}

/// # Examples
///
/// ```
/// use std::default::Default;
/// use w3c_validators::NuValidator;
/// let nu = NuValidator::new(Default::default());
/// match nu.validate_text("<!DOCTYPE html><html lang=en></html>") {
///     Some(result) => {
///         if !result.is_valid() {
///             for msg in result.messages {
///                 println!("{}: {}", msg._type, msg.message);
///             }
///         }
///     }
///     None => {},
/// }
/// ```
impl NuValidator {
    pub fn new(opts: NuValidatorOpts) -> NuValidator {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("HttpClient failed to construct");
        NuValidator {
            validator_uri: opts.validator_uri,
            http_client,
        }
    }

    fn fetch_result(&self, mut response: Response) -> Option<NuValidatorResult> {
        match response.text() {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn validate_uri(&self, uri: &str) -> Option<NuValidatorResult> {
        let url = &format!("{}?doc={}&out=json", MARKUP_VALIDATOR_URI, uri);
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

    pub fn validate_text(&self, text: &'static str) -> Option<NuValidatorResult> {
        let url = &format!("{}?out=json", MARKUP_VALIDATOR_URI);
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(ua_validator!()));
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(TEXT_HTML_UTF_8).expect(""),
        );
        match self
            .http_client
            .post(url)
            .headers(headers)
            .body(text)
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
