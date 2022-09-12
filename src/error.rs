pub type Result<T> = std::result::Result<T, CompileError>;
use crate::token::{Token, TokenKind};
use std::fmt::Display;

#[derive(Debug)]
pub enum CompileError {
    FmtError(std::fmt::Error),
    Unexpected { expect: TokenKind, result: Token },
    Expected(TokenKind),
    ParseError,
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unexpected { expect, result } => {
                write!(f, "expect: {}, find: {}", expect, result)
            }
            Self::Expected(expect) => write!(f, "expect {}", expect),
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
