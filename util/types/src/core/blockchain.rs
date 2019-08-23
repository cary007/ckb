use failure::{err_msg, Error as FailureError};
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptHashType {
    Data = 0,
    Type = 1,
}

impl Default for ScriptHashType {
    fn default() -> Self {
        ScriptHashType::Data
    }
}

impl TryFrom<u8> for ScriptHashType {
    type Error = FailureError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ScriptHashType::Data),
            1 => Ok(ScriptHashType::Type),
            _ => Err(err_msg(format!("Invalid script hash type {}", v))),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepType {
    Code = 0,
    DepGroup = 1,
}

impl Default for DepType {
    fn default() -> Self {
        DepType::Code
    }
}

impl TryFrom<u8> for DepType {
    type Error = FailureError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(DepType::Code),
            1 => Ok(DepType::DepGroup),
            _ => Err(err_msg(format!("Invalid dep type {}", v))),
        }
    }
}
