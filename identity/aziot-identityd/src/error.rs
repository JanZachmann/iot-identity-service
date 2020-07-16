// Copyright (c) Microsoft. All rights reserved.

#[derive(Debug)]
pub enum Error {
    Internal(InternalError),
    InvalidParameter(&'static str, Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(_) => f.write_str("internal error"),
            Error::InvalidParameter(name, _) => write!(f, "parameter {:?} has an invalid value", name),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Internal(err) => Some(err),
            Error::InvalidParameter(_, err) => Some(&**err),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum InternalError {
    InitializeCommon(aziot_common::error::Error),
    LoadSettings(std::io::Error),
    ParseSettings(toml::de::Error),
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::InitializeCommon(_) => f.write_str("could not load common settings"),
            InternalError::LoadSettings(_) => f.write_str("could not load settings"),
            InternalError::ParseSettings(_) => f.write_str("could not parse settings"),
        }
    }
}

impl std::error::Error for InternalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InternalError::InitializeCommon(err) => Some(err),
            InternalError::LoadSettings(err) => Some(err),
            InternalError::ParseSettings(err) => Some(err),
        }
    }
}

impl From<aziot_common::error::Error> for InternalError {
    fn from(err: aziot_common::error::Error) -> Self {
        InternalError::InitializeCommon(err)
    }
}