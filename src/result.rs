use crate::token::{Token, TokenKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("{}/{}: expect {}, found {}", stop.row(), stop.col(), expect, result)]
    Unexpected {
        stop: Token,
        expect: TokenKind,
        result: TokenKind,
    },
    #[error("{}/{}: expect {}", stop.row(), stop.col(), expect)]
    Expected { stop: Token, expect: TokenKind },
    #[error("ParseError")]
    ParseError(Option<&'static str>),
}
