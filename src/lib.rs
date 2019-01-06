//! Rust wrapper for the World Wide Web Consortium’s online validation services.
#![doc(html_root_url = "https://docs.rs/w3c_validators/0.1.0")]
#![deny(warnings)]
#![crate_name = "w3c_validators"]
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

#[macro_export]
macro_rules! _version_string {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[macro_export]
macro_rules! ua_validator {
    () => {
        concat!(
            "w3c_validators v",
            _version_string!(),
            " (https://crates.io/crates/w3c_validators)"
        )
    };
}

pub use w3c_validators::{
    CssValidator, CssValidatorResult, NuValidator, NuValidatorResult, CSS_VALIDATOR_URI,
    MARKUP_VALIDATOR_URI,
};
pub mod w3c_validators;
