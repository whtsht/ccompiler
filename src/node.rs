use crate::consume;
use crate::error::{CompileError, Result};
use crate::token::{Operation, Token, TokenKind};
use std::fmt::Display;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub struct Node {
    col: u32,
    row: u32,
    token: TokenKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    pub fn new(col: u32, row: u32, token: TokenKind) -> Self {
        Self {
            col,
            row,
            token,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_op(col: u32, row: u32, token: TokenKind, lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self {
            col,
            row,
            token,
            lhs: Some(lhs),
            rhs: Some(rhs),
        }
    }

    pub fn new_num(col: u32, row: u32, num: u32) -> Self {
        Self {
            col,
            row,
            token: TokenKind::NUM(num),
            lhs: None,
            rhs: None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} : {}", self.row, self.row, self.token)
    }
}
