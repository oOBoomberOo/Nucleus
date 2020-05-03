use dialoguer::Validator;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

lazy_static! {
	static ref NAMESPACE_VALIDATOR: Regex = Regex::new(r#"^[a-z0-9_\.\-:]+$"#).unwrap();
	static ref NAME_VALIDATOR: Regex = Regex::new(r#"^\w{3,16}$"#).unwrap();
	static ref DATAPACK_NAME_VALIDATOR: Regex = Regex::new(r#"^[\w\d\s]+$"#).unwrap();
}

#[derive(Debug, Error)]
pub enum ValidatorError {
	#[error("Namespace '{0}' must follow this regex: /{}/", NAMESPACE_VALIDATOR.as_str())]
	InvalidNamespace(String),
	#[error("Name '{0}' must be within 3-16 character and must all be english characters")]
	InvalidName(String),
	#[error("Datapack Name '{0}' contain non-english character or special symbol")]
	InvalidDatapackName(String),
}

/// Validating Namespace ID
pub struct NamespaceValidator;

impl Validator for NamespaceValidator {
	type Err = ValidatorError;
	fn validate(&self, text: &str) -> Result<(), Self::Err> {
		if NAMESPACE_VALIDATOR.is_match(&text) {
			Ok(())
		}
		else {
			Err(ValidatorError::InvalidNamespace(text.to_owned()))
		}
	}
}

/// Validating Player name
pub struct NameValidator;

impl Validator for NameValidator {
	type Err = ValidatorError;
	fn validate(&self, text: &str) -> Result<(), Self::Err> {
		if NAME_VALIDATOR.is_match(&text) {
			Ok(())
		}
		else {
			Err(ValidatorError::InvalidName(text.to_owned()))
		}
	}
}

/// Validating Datapack Name that *could* be transformed into namespace
pub struct DatapackNameValidator;

impl Validator for DatapackNameValidator {
	type Err = ValidatorError;
	fn validate(&self, text: &str) -> Result<(), Self::Err> {
		if DATAPACK_NAME_VALIDATOR.is_match(&text) {
			Ok(())
		}
		else {
			Err(ValidatorError::InvalidDatapackName(text.to_owned()))
		}
	}
}
