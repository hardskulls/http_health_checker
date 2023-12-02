use std::process::{ExitCode, Termination};
use thiserror::Error;

pub type DefaultErr = String;

pub type Result<T, E = DefaultErr> = std::result::Result<T, E>;

pub struct Empty;

impl Termination for Empty {
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}

/// This error means that url is invalid;
#[derive(Error, Debug)]
#[error("URL parsing error")]
pub struct UrlParseErr;

impl From<UrlParseErr> for String {
    fn from(value: UrlParseErr) -> Self {
        value.to_string()
    }
}
