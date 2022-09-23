use crate::error::{CResult, CompileError};
use std::collections::HashMap;
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
    /// Local variable | (a..z | A..Z | _)(a..z | A..Z | _ | 0..9)*
    LocalVar { symbol: String, offset: u32 },
    /// Semicolon | ;
    Semicolon,
    /// Assign | =
    Assign,
    /// Return | return
    Return,
    /// If,
    If,
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
            | TokenKind::If
            | TokenKind::GreaterOrEqual => 2,
            TokenKind::Num(num) => digits(*num),
            TokenKind::LocalVar { symbol, .. } => symbol.len() as u32,
            TokenKind::Return => 6,
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
            TokenKind::If => write!(f, "If"),
            TokenKind::Return => write!(f, "Return"),
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
        Self { col, row, kind }
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
        .or_else(|_| Err(CompileError::ParseError(Some("Number"))))?;
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

fn var_token(line: &str) -> Option<(String, usize)> {
    if is_var_first(line.chars().next()?) {
        let var = line.split_once(|c| !is_alnum(c)).unwrap().0;
        Some((var.to_string(), var.len()))
    } else {
        None
    }
}

fn is_var_first(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}

fn is_alnum(c: char) -> bool {
    is_var_first(c) || '0' <= c && c <= '9'
}

#[test]
fn test_match_word() {
    let a = "Hello1 = 3;";
    assert_eq!(a.split_once(|c| !is_alnum(c)).unwrap().0, "Hello1");
}

#[derive(Debug)]
pub struct TokenStream {
    token: Token,
    stream: Peekable<IntoIter<Token>>,
}

pub fn return_token(tokens: &mut Vec<Token>, line: &String, row: usize, col: &mut usize) -> bool {
    match &line[*col..*col + 6] {
        "return" if !is_alnum(line[*col + 6..].chars().next().unwrap()) => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::Return));
            *col += 6;
            true
        }
        _ => two_word_token(tokens, line, row, col),
    }
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
        "if" => {
            tokens.push(Token::new(*col as u32, row as u32, TokenKind::If));
            *col += 2;
            true
        }
        _ => one_word_token(tokens, line, row, col),
    }
}

pub fn one_word_token(tokens: &mut Vec<Token>, line: &String, row: usize, col: &mut usize) -> bool {
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
        _ => return false,
    }
    true
}

fn other_word_token(
    tokens: &mut Vec<Token>,
    line: &String,
    variables: &mut HashMap<String, u32>,
    row: usize,
    col: &mut usize,
) -> CResult<()> {
    if let Some((var, len)) = var_token(&line[*col..]) {
        if let Some(&offset) = variables.get(&var) {
            tokens.push(Token::new(
                *col as u32,
                row as u32,
                TokenKind::LocalVar {
                    symbol: var,
                    offset,
                },
            ));
            *col += len;
        } else {
            let offset = (variables.len() as u32 + 1) * 8;
            tokens.push(Token::new(
                *col as u32,
                row as u32,
                TokenKind::LocalVar {
                    symbol: var.clone(),
                    offset,
                },
            ));
            *col += len;
            variables.insert(var, offset);
        }
    } else {
        let (num, count) = num_token(&line[*col..])?;
        tokens.push(Token::new(*col as u32, row as u32, TokenKind::Num(num)));
        *col += count;
    }

    Ok(())
}

pub fn tokenize(source: Vec<String>) -> CResult<TokenStream> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut variables: HashMap<String, u32> = HashMap::new();

    // If source code is empty, return parse error.
    if source.is_empty() {
        return Err(CompileError::ParseError(Some("A Source code is empty")));
    }

    for (row, line) in source.into_iter().enumerate() {
        let mut col = 0;
        let max = line.len();
        loop {
            let result = match max - col {
                0 => break,
                1 => one_word_token(&mut tokens, &line, row, &mut col),
                2..=6 => two_word_token(&mut tokens, &line, row, &mut col),
                _ => return_token(&mut tokens, &line, row, &mut col),
            };

            if !result {
                other_word_token(&mut tokens, &line, &mut variables, row, &mut col)?;
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
            assert_eq!(result, expect, "{}", source);
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

    let expect = vec![
        Token::new(
            0,
            0,
            TokenKind::LocalVar {
                symbol: "hello1".into(),
                offset: 8,
            },
        ),
        Token::new(7, 0, TokenKind::Assign),
        Token::new(9, 0, TokenKind::Num(3)),
        Token::new(10, 0, TokenKind::Semicolon),
    ];
    test_tokenize("hello1 = 3;", expect);

    let expect = vec![
        Token::new(0, 0, TokenKind::Return),
        Token::new(7, 0, TokenKind::Num(8)),
        Token::new(8, 0, TokenKind::Semicolon),
    ];
    test_tokenize("return 8;", expect);

    let expect = vec![Token::new(
        0,
        0,
        TokenKind::LocalVar {
            symbol: "returned".into(),
            offset: 8,
        },
    )];
    test_tokenize("returned;", expect);

    let expect = vec![
        Token::new(0, 0, TokenKind::If),
        Token::new(3, 0, TokenKind::LRoundBracket),
        Token::new(4, 0, TokenKind::Num(1)),
        Token::new(5, 0, TokenKind::RRoundBracket),
        Token::new(
            7,
            0,
            TokenKind::LocalVar {
                symbol: "b".into(),
                offset: 8,
            },
        ),
        Token::new(9, 0, TokenKind::Assign),
        Token::new(11, 0, TokenKind::Num(20)),
        Token::new(13, 0, TokenKind::Semicolon),
    ];
    test_tokenize("if (1) b = 20;", expect);
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
