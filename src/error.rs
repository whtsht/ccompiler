pub type Result<T> = std::result::Result<T, CompileError>;
use crate::token::{Token, TokenKind};
use std::fmt::Display;

#[derive(Debug)]
pub enum CompileError {
    FmtError(std::fmt::Error),
    Unexpected {
        stop: Token,
        expect: TokenKind,
        result: TokenKind,
    },
    Expected {
        stop: Token,
        expect: TokenKind,
    },
    ParseError,
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected {
                stop,
                expect,
                result,
            } => {
                write!(
                    f,
                    "{}:{} expect: {}, find: {}",
                    stop.row(),
                    stop.col() + stop.len(),
                    expect,
                    result
                )
            }
            Self::Expected { stop, expect } => {
                write!(
                    f,
                    "{}:{} expect {}",
                    stop.row(),
                    stop.col() + stop.len(),
                    expect
                )
            }
            Self::ParseError => write!(f, "parse error"),
            Self::FmtError(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::fmt::Error> for CompileError {
    fn from(err: std::fmt::Error) -> Self {
        CompileError::FmtError(err)
    }
}
