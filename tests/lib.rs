extern crate w3c_validators;

use std::default::Default;
use w3c_validators::{CssValidator, NuValidator, CSS_VALIDATOR_URI, MARKUP_VALIDATOR_URI};

#[test]
fn test_nu_validator() {
    let nu = NuValidator::new(Default::default());
    assert_eq!(nu.validator_uri, MARKUP_VALIDATOR_URI);
    match nu.validate_uri("https://www.spkdev.net/") {
        Some(result) => {
            assert!(result.is_valid());
        }
        None => assert!(false),
    }
    match nu.validate_text("<!DOCTYPE html><html lang=en></html>") {
        Some(result) => {
            assert!(!result.is_valid());
            for msg in result.messages {
                assert!(msg.is_error());
            }
        }
        None => assert!(false),
    }
}

#[test]
fn test_css_validator() {
    let nu = CssValidator::new(Default::default());
    assert_eq!(nu.validator_uri, CSS_VALIDATOR_URI);
    match nu.validate_uri("https://www.spkdev.net/assets/main.css") {
        Some(result) => {
            assert!(result.is_valid());
        }
        None => assert!(false),
    }
    match nu.validate_text("tbody th{width: /* 25%} */") {
        Some(result) => {
            assert!(!result.is_valid());
            for msg in result.cssvalidation.errors.unwrap() {
                assert!(!msg.message.is_empty());
            }
        }
        None => assert!(false),
    }
}
