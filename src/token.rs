use crate::error::{CompileError, Result};
use crate::to_digits;
use std::fmt::Display;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Add, //      +
    Sub, //      -
    Mul, //      *
    Div, //      /
    Lbr, //      (
    Rbr, //      )
    Num(u32),
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&TokenKind::Num(_), &TokenKind::Num(_)) => true,
            (&TokenKind::Add, &TokenKind::Add)
            | (&TokenKind::Sub, &TokenKind::Sub)
            | (&TokenKind::Mul, &TokenKind::Mul)
            | (&TokenKind::Div, &TokenKind::Div)
            | (&TokenKind::Lbr, &TokenKind::Lbr)
            | (&TokenKind::Rbr, &TokenKind::Rbr) => true,
            _ => false,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Num(..) => write!(f, "Number"),
            TokenKind::Add => write!(f, "Operation: +"),
            TokenKind::Sub => write!(f, "Operation: -"),
            TokenKind::Mul => write!(f, "Operation: *"),
            TokenKind::Div => write!(f, "Operation: /"),
            TokenKind::Lbr => write!(f, "Operation: ("),
            TokenKind::Rbr => write!(f, "Operation: )"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    col: u32,
    row: u32,
    len: u32,
    kind: TokenKind,
}

impl Token {
    pub fn new(col: u32, row: u32, len: u32, kind: TokenKind) -> Self {
        Self {
            col,
            row,
            len,
            kind,
        }
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn row(&self) -> u32 {
        self.row
    }

    pub fn len(&self) -> u32 {
        self.len
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

#[derive(Debug)]
pub struct TokenStream {
    token: Token,
    stream: Peekable<IntoIter<Token>>,
}

pub fn tokenize<I: Iterator<Item = char>>(mut source: Peekable<I>) -> Result<TokenStream> {
    let mut tokens = Vec::new();
    let mut row = 1;
    let mut col = 1;

    // If source code is empty, return parse error.
    if source.peek().is_none() {
        return Err(CompileError::ParseError);
    }

    while let Some(&s) = source.peek() {
        match s {
            '+' => {
                tokens.push(Token::new(col, row, 1, TokenKind::Add.into()));
                source.next();
                col += 1;
            }
            '-' => {
                tokens.push(Token::new(col, row, 1, TokenKind::Sub.into()));
                source.next();
                col += 1;
            }
            '*' => {
                tokens.push(Token::new(col, row, 1, TokenKind::Mul.into()));
                source.next();
                col += 1;
            }
            '/' => {
                tokens.push(Token::new(col, row, 1, TokenKind::Div.into()));
                source.next();
                col += 1;
            }
            '(' => {
                tokens.push(Token::new(col, row, 1, TokenKind::Lbr.into()));
                source.next();
                col += 1;
            }
            ')' => {
                tokens.push(Token::new(col, row, 1, TokenKind::Rbr.into()));
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
                if let Some((num, count)) = to_digits(&mut source) {
                    tokens.push(Token::new(col, row, count, TokenKind::Num(num)));
                    col += count;
                } else {
                    return Err(CompileError::ParseError);
                }
            }
        }
    }

    Ok(TokenStream {
        token: tokens[0].clone(),
        stream: tokens.into_iter().peekable(),
    })
}

#[test]
fn testrunner_tokenize() {
    let test_tokenize = |source: &str, expect: Vec<Token>| {
        let token_stream = tokenize(source.chars().peekable()).unwrap();

        for (expect, result) in expect.into_iter().zip(token_stream.stream) {
            assert_eq!(expect, result, "{}", source);
        }
    };
    let expect = vec![
        Token::new(1, 1, 1, TokenKind::Num(1)),
        Token::new(2, 1, 1, TokenKind::Add),
        Token::new(3, 1, 1, TokenKind::Num(4)),
    ];
    test_tokenize("1+4", expect);

    let expect = vec![
        Token::new(1, 1, 1, TokenKind::Sub),
        Token::new(2, 1, 2, TokenKind::Num(23)),
        Token::new(4, 1, 1, TokenKind::Add),
    ];
    test_tokenize("-23+", expect);

    let expect = vec![
        Token::new(1, 1, 1, TokenKind::Num(7)),
        Token::new(2, 1, 1, TokenKind::Add),
        Token::new(3, 1, 1, TokenKind::Num(3)),
        Token::new(1, 2, 1, TokenKind::Sub),
        Token::new(2, 2, 1, TokenKind::Num(4)),
    ];
    test_tokenize("7+3\n-4", expect);
}

impl TokenStream {
    pub fn expect(&mut self, expect: TokenKind) -> Result<()> {
        if let Some(token) = self.stream.next() {
            if token.kind() == expect {
                Ok(())
            } else {
                Err(CompileError::Unexpected {
                    stop: self.token.clone(),
                    expect,
                    result: token.kind(),
                })
            }
        } else {
            Err(CompileError::Expected {
                stop: self.token.clone(),
                expect,
            })
        }
    }

    pub fn expect_number(&mut self) -> Result<u32> {
        if let Some(token) = self.stream.next() {
            if let TokenKind::Num(num) = token.kind() {
                self.token = token.clone();
                Ok(num)
            } else {
                Err(CompileError::Unexpected {
                    stop: self.token.clone(),
                    expect: TokenKind::Num(0),
                    result: token.kind(),
                })
            }
        } else {
            Err(CompileError::Expected {
                stop: self.token.clone(),
                expect: TokenKind::Num(0),
            })
        }
    }

    pub fn consume(&mut self, expect: TokenKind) -> bool {
        if let Some(token) = self.stream.peek() {
            if expect == token.kind() {
                self.token = self.stream.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in self.stream.clone() {
            write!(f, "{:?} ", token)?;
        }
        write!(f, "\n")
    }
}
