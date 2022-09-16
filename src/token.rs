use crate::error::{CResult, CompileError};
use std::fmt::Display;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub enum TokenKind {
    /// Additional | +
    Add,
    /// Subtraction | -
    Sub,
    /// Multiplication | *
    Mul,
    /// Division | /
    Div,
    /// Left hand Bracket | (
    LRoundBracket,
    /// Right hand Bracket | )
    RRoundBracket,
    /// Equal to | ==
    Equal,
    /// Not equal to | !=
    NEqual,
    /// Less than | <
    Less,
    /// Greater than | >
    Greater,
    /// Less than or equal to | <=
    LessOrEqual,
    /// Greater than or equal to | >=
    GreaterOrEqual,
    Num(u32),
}

fn digits(mut x: u32) -> u32 {
    let mut count = 0;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    count
}

#[test]
fn test_digits() {
    assert_eq!(digits(32), 2);
    assert_eq!(digits(10923480), 8);
    assert_eq!(digits(0), 0);
}

impl TokenKind {
    pub fn len(&self) -> u32 {
        match self {
            TokenKind::Add | TokenKind::Sub | TokenKind::Mul | TokenKind::Div => 1,
            TokenKind::LRoundBracket | TokenKind::RRoundBracket => 1,
            TokenKind::Less | TokenKind::Greater => 1,
            TokenKind::Equal
            | TokenKind::NEqual
            | TokenKind::LessOrEqual
            | TokenKind::GreaterOrEqual => 2,
            TokenKind::Num(num) => digits(*num),
        }
    }
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&TokenKind::Num(_), &TokenKind::Num(_)) => true,
            (&TokenKind::Add, &TokenKind::Add)
            | (&TokenKind::Sub, &TokenKind::Sub)
            | (&TokenKind::Mul, &TokenKind::Mul)
            | (&TokenKind::Div, &TokenKind::Div)
            | (&TokenKind::LRoundBracket, &TokenKind::LRoundBracket)
            | (&TokenKind::RRoundBracket, &TokenKind::RRoundBracket)
            | (&TokenKind::Equal, &TokenKind::Equal)
            | (&TokenKind::NEqual, &TokenKind::NEqual)
            | (&TokenKind::Less, &TokenKind::Less)
            | (&TokenKind::Greater, &TokenKind::Greater)
            | (&TokenKind::LessOrEqual, &TokenKind::LessOrEqual)
            | (&TokenKind::GreaterOrEqual, &TokenKind::GreaterOrEqual) => true,
            _ => false,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Num(_) => write!(f, "Number"),
            TokenKind::Add => write!(f, "Operation: +"),
            TokenKind::Sub => write!(f, "Operation: -"),
            TokenKind::Mul => write!(f, "Operation: *"),
            TokenKind::Div => write!(f, "Operation: /"),
            TokenKind::LRoundBracket => write!(f, "Operation: ("),
            TokenKind::RRoundBracket => write!(f, "Operation: )"),
            TokenKind::Equal => write!(f, "Operation: =="),
            TokenKind::NEqual => write!(f, "Operation: !="),
            TokenKind::Less => write!(f, "Operation: <"),
            TokenKind::Greater => write!(f, "Operation: >"),
            TokenKind::LessOrEqual => write!(f, "Operation: <="),
            TokenKind::GreaterOrEqual => write!(f, "Operation: >="),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    col: u32,
    row: u32,
    kind: TokenKind,
}

impl Token {
    pub fn new(col: u32, row: u32, kind: TokenKind) -> Self {
        Self {
            col: col + 1,
            row: row + 1,
            kind,
        }
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn row(&self) -> u32 {
        self.row
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

fn num_token(s: &str) -> CResult<(u32, usize)> {
    let mut result = (&s[0..1])
        .parse::<u32>()
        .or_else(|_| Err(CompileError::ParseError))?;
    if s.len() <= 1 {
        return Ok((result, 1));
    }
    let mut count = 1;

    while let Ok(num) = (&s[count..count + 1]).parse::<u32>() {
        result = result * 10 + num;
        count += 1;
        if count >= s.len() {
            break;
        }
    }
    Ok((result, count))
}

#[derive(Debug)]
pub struct TokenStream {
    token: Token,
    stream: Peekable<IntoIter<Token>>,
}

pub fn tokenize(source: Vec<String>) -> CResult<TokenStream> {
    let mut tokens: Vec<Token> = Vec::new();

    // If source code is empty, return parse error.
    if source.is_empty() {
        return Err(CompileError::ParseError);
    }

    for (row, line) in source.into_iter().enumerate() {
        let mut col = 0;
        let max = line.len() - 1;
        while col < max {
            match &line[col..col + 2] {
                "==" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Equal));
                    col += 2;
                    continue;
                }
                "!=" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::NEqual));
                    col += 2;
                    continue;
                }
                ">=" => {
                    tokens.push(Token::new(
                        col as u32,
                        row as u32,
                        TokenKind::GreaterOrEqual,
                    ));
                    col += 2;
                    continue;
                }
                "<=" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::LessOrEqual));
                    col += 2;
                    continue;
                }
                _ => {}
            }
            match &line[col..col + 1] {
                " " => {}
                "<" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Less));
                }
                ">" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Greater));
                }
                "+" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Add));
                }
                "-" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Sub));
                }
                "*" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Mul));
                }
                "/" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Div));
                }
                "(" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::LRoundBracket));
                }
                ")" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::RRoundBracket));
                }
                _ => {
                    let (num, count) = num_token(&line[col..])?;
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Num(num)));
                    col += count;
                    continue;
                }
            }

            col += 1;
        }

        if col < line.len() {
            match &line[col..col + 1] {
                " " => {}
                "<" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Less));
                }
                ">" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Greater));
                }
                "+" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Add));
                }
                "-" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Sub));
                }
                "*" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Mul));
                }
                "/" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Div));
                }
                "(" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::LRoundBracket));
                }
                ")" => {
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::RRoundBracket));
                }
                _ => {
                    let (num, _) = num_token(&line[col..])?;
                    tokens.push(Token::new(col as u32, row as u32, TokenKind::Num(num)));
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
        let token_stream: TokenStream = tokenize(vec![source.to_string()]).unwrap();

        for (expect, result) in expect.into_iter().zip(token_stream.stream) {
            assert_eq!(expect, result, "{}", source);
        }
    };
    let expect = vec![
        Token::new(0, 0, TokenKind::Num(1)),
        Token::new(1, 0, TokenKind::Add),
        Token::new(2, 0, TokenKind::Num(4)),
    ];
    test_tokenize("1+4", expect);

    let expect = vec![
        Token::new(0, 0, TokenKind::Sub),
        Token::new(1, 0, TokenKind::Num(23)),
        Token::new(3, 0, TokenKind::Add),
    ];
    test_tokenize("-23+", expect);

    let expect = vec![
        Token::new(0, 0, TokenKind::Num(7)),
        Token::new(1, 0, TokenKind::Add),
        Token::new(2, 0, TokenKind::Num(3)),
        Token::new(3, 0, TokenKind::Sub),
        Token::new(4, 0, TokenKind::Num(4)),
    ];
    test_tokenize("7+3-4", expect);
}

impl TokenStream {
    pub fn expect(&mut self, expect: TokenKind) -> CResult<()> {
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

    pub fn expect_number(&mut self) -> CResult<u32> {
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
