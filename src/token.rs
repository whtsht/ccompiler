use crate::error::{CompileError, Result};
use crate::to_digits;
use std::fmt::Display;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OP(Operation),
    NUM(u32),
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::NUM(..) => write!(f, "Number"),
            TokenKind::OP(op) => match op {
                Operation::Add => write!(f, "Operation: +"),
                Operation::Sub => write!(f, "Operation: -"),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Add,
    Sub,
}

impl From<u32> for TokenKind {
    fn from(num: u32) -> Self {
        TokenKind::NUM(num)
    }
}

impl From<Operation> for TokenKind {
    fn from(op: Operation) -> Self {
        TokenKind::OP(op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    col: u32,
    row: u32,
    kind: TokenKind,
}

impl Token {
    pub fn new(col: u32, row: u32, kind: TokenKind) -> Self {
        Self { col, row, kind }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} : {}", self.row, self.row, self.kind)
    }
}

pub fn tokenize<I: Iterator<Item = char>>(source: &mut Peekable<I>) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut row = 1;
    let mut col = 1;

    while let Some(&s) = source.peek() {
        match s {
            '+' => {
                tokens.push(Token::new(col, row, Operation::Add.into()));
                source.next();
                col += 1;
            }
            '-' => {
                tokens.push(Token::new(col, row, Operation::Sub.into()));
                source.next();
                col += 1;
            }
            ' ' => {
                col += 1;
                source.next();
            }
            '\n' => {
                col = 1;
                row += 1;
                source.next();
            }
            _ => {
                if let Some((num, count)) = to_digits(source) {
                    tokens.push(Token::new(col, row, num.into()));
                    col += count;
                } else {
                    return Err(CompileError::ParseError);
                }
            }
        }
    }

    Ok(tokens)
}
