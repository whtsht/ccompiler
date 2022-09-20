use crate::error::{CResult, CompileError};
use std::fmt::Display;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
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
    /// Number | 1, 2, ... , 255
    Num(u32),
    /// Local variable
    LocalVar { symbol: String, offset: u32 },
    /// Semicolon | ;
    Semicolon,
    /// Assign | =
    Assign,
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
            TokenKind::Semicolon | TokenKind::Assign => 1,
            TokenKind::Equal
            | TokenKind::NEqual
            | TokenKind::LessOrEqual
            | TokenKind::GreaterOrEqual => 2,
            TokenKind::Num(num) => digits(*num),
            TokenKind::LocalVar { symbol, .. } => symbol.len() as u32,
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
            TokenKind::LocalVar { .. } => write!(f, "Local variable"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Assign => write!(f, "Assign"),
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

pub fn two_word_token(tokens: &mut Vec<Token>, line: &String, row: usize, col: &mut usize) -> bool {
    match &line[*col..*col + 2] {
        "==" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Equal));
            *col += 2;
            true
        }
        "!=" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::NEqual));
            *col += 2;
            true
        }
        ">=" => {
            tokens.push(Token::new(
                *col as u32,
                row as u32,
                TokenKind::GreaterOrEqual,
            ));
            *col += 2;
            true
        }
        "<=" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::LessOrEqual));
            *col += 2;
            true
        }
        _ => false,
    }
}

pub fn one_word_token(
    tokens: &mut Vec<Token>,
    line: &String,
    row: usize,
    col: &mut usize,
) -> CResult<()> {
    let word = &line[*col..*col + 1];
    match word {
        " " => {
            *col += 1;
        }
        "<" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Less));
            *col += 1;
        }
        ">" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Greater));
            *col += 1;
        }
        "+" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Add));
            *col += 1;
        }
        "-" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Sub));
            *col += 1;
        }
        "*" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Mul));
            *col += 1;
        }
        "/" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Div));
            *col += 1;
        }
        "(" => {
            tokens.push(Token::new(
                *col as u32,
                row as u32,
                TokenKind::LRoundBracket,
            ));
            *col += 1;
        }
        ")" => {
            tokens.push(Token::new(
                *col as u32,
                row as u32,
                TokenKind::RRoundBracket,
            ));
            *col += 1;
        }
        ";" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Semicolon));
            *col += 1;
        }
        "=" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Assign));
            *col += 1;
        }
        _ => {
            if word.chars().all(char::is_alphabetic) {
                tokens.push(Token::new(
                    *col as u32,
                    row as u32,
                    TokenKind::LocalVar {
                        symbol: word.to_string(),
                        offset: (word.chars().next().unwrap() as u8 - b'a' + 1) as u32 * 8,
                    },
                ));
                *col += 1;
            } else {
                let (num, count) = num_token(&line[*col..])?;
                tokens.push(Token::new(*col as u32, row as u32, TokenKind::Num(num)));
                *col += count;
            }
        }
    }
    Ok(())
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
            if two_word_token(&mut tokens, &line, row, &mut col) {
                continue;
            }
            one_word_token(&mut tokens, &line, row, &mut col)?;
        }

        if col < line.len() {
            one_word_token(&mut tokens, &line, row, &mut col)?;
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

    let expect = vec![
        Token::new(
            0,
            0,
            TokenKind::LocalVar {
                symbol: String::from("a"),
                offset: 8,
            },
        ),
        Token::new(2, 0, TokenKind::Assign),
        Token::new(4, 0, TokenKind::Num(3)),
    ];

    test_tokenize("a = 3", expect);
}

impl TokenStream {
    pub fn is_empty(&mut self) -> bool {
        self.stream.peek().is_none()
    }

    pub fn expect(&mut self, expect: TokenKind) -> CResult<TokenKind> {
        if let Some(token) = self.stream.next() {
            if token.kind() == expect {
                self.token = token.clone();
                Ok(token.kind())
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

    pub fn expect_local_variable(&mut self) -> CResult<TokenKind> {
        if let Some(token) = self.stream.peek() {
            if let TokenKind::LocalVar { .. } = token.kind() {
                self.token = token.clone();
                let kind = token.kind();
                self.stream.next();
                Ok(kind)
            } else {
                Err(CompileError::Unexpected {
                    stop: self.token.clone(),
                    expect: TokenKind::LocalVar {
                        symbol: String::new(),
                        offset: 0,
                    },
                    result: token.kind(),
                })
            }
        } else {
            Err(CompileError::Expected {
                stop: self.token.clone(),
                expect: TokenKind::LocalVar {
                    symbol: String::new(),
                    offset: 0,
                },
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
