use thiserror::Error;

pub trait Validator<T: ?Sized> {
    type Error: std::error::Error;

    fn validate<'a>(&'a self, value: &'a T) -> Result<(), Self::Error>;
}

#[derive(Error, Debug)]
pub enum EmptyStrValidatorError {
    #[error("Empty error")]
    Empty,
}

pub struct EmptyStrValidator;
impl Default for EmptyStrValidator {
    fn default() -> Self {
        Self {}
    }
}

impl Validator<str> for EmptyStrValidator {
    type Error = EmptyStrValidatorError;

    fn validate<'a>(&'a self, _: &'a str) -> Result<(), Self::Error> { Ok(()) }
}

#[derive(Error, Debug)]
pub enum StrValidatorError {
    #[error("Max limit")]
    MaxLimit,
    #[error("Min limit")]
    MinLimit,
}
pub struct StrValidator {
    pub min_length: usize,
    pub max_length: usize,
}

impl StrValidator {
    pub fn new(min_length: usize, max_length: usize) -> Self { Self { max_length, min_length } }
}

impl Default for StrValidator {
    fn default() -> Self {
        Self { max_length: usize::MAX - 1, min_length: 0 }
    }
}

impl Validator<str> for StrValidator {
    type Error = StrValidatorError;
    fn validate<'a>(&'a self, value: &'a str) -> Result<(), Self::Error> {
        let len = value.len();
        if len > self.max_length {
            return Err(StrValidatorError::MaxLimit);
        }
        if len <= self.min_length {
            return Err(StrValidatorError::MinLimit);
        }
        Ok(())
    }
}


