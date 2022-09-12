use std::fmt::{Display, Write};
use std::iter::{Iterator, Peekable};

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

pub type Result<T> = std::result::Result<T, CompileError>;

pub fn to_num<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<u32> {
    let mut result = iter.next()?.to_digit(10)? as u32;
    loop {
        match iter.peek() {
            Some(c) => match c.to_digit(10) {
                Some(i) => {
                    result = result * 10 + i as u32;
                    iter.next();
                }
                None => break,
            },
            None => break,
        }
    }
    Some(result)
}

pub fn to_digits<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<(u32, u32)> {
    let mut result = iter.next()?.to_digit(10)? as u32;
    let mut count = 1;
    loop {
        match iter.peek() {
            Some(c) => match c.to_digit(10) {
                Some(i) => {
                    result = result * 10 + i as u32;
                    iter.next();
                    count += 1;
                }
                None => break,
            },
            None => break,
        }
    }
    Some((result, count))
}

#[test]
fn test_to_num() {
    let testcase = vec![
        ("21", Some(21)),
        ("+", None),
        ("+32", None),
        ("32+", Some(32)),
    ];

    for (input, expected) in testcase {
        let mut iter = input.chars().peekable();
        assert_eq!(to_num(&mut iter), expected);
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} : {}", self.row, self.row, self.kind)
    }
}

fn tokenize<I: Iterator<Item = char>>(source: &mut Peekable<I>) -> Result<Vec<Token>> {
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

fn expect<I: Iterator<Item = Token>>(token: &mut I, expected: Operation) -> Result<Operation> {
    if let Some(t) = token.next() {
        match t.kind {
            TokenKind::OP(operation) if operation == expected => return Ok(operation),
            TokenKind::OP(..) => {
                return Err(CompileError::Unexpected {
                    expect: TokenKind::OP(expected),
                    result: t,
                })
            }
            TokenKind::NUM(num) => {
                return Err(CompileError::Unexpected {
                    expect: TokenKind::NUM(num),
                    result: t,
                })
            }
        }
    }
    Err(CompileError::ParseError)
}

fn expect_number<I: Iterator<Item = Token>>(token: &mut I) -> Result<u32> {
    match token.next() {
        Some(t) => match t.kind {
            TokenKind::NUM(num) => Ok(num),
            TokenKind::OP(..) => Err(CompileError::Unexpected {
                expect: TokenKind::NUM(0),
                result: t,
            }),
        },
        None => Err(CompileError::Expected(TokenKind::NUM(0))),
    }
}

fn consume<I: Iterator<Item = Token>>(token: &mut Peekable<I>, expected: Operation) -> bool {
    if let Some(t) = token.peek() {
        match &t.kind {
            TokenKind::OP(operation) if operation == &expected => {
                token.next();
                return true;
            }
            _ => return false,
        }
    }
    false
}

#[test]
fn test_tokenize() {
    let mut source = "1+4".chars().peekable();
    let tokens = tokenize(&mut source).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(1, 1, TokenKind::NUM(1)),
            Token::new(2, 1, TokenKind::OP(Operation::Add)),
            Token::new(3, 1, TokenKind::NUM(4)),
        ]
    );

    let mut source = "-23+".chars().peekable();
    let tokens = tokenize(&mut source).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(1, 1, TokenKind::OP(Operation::Sub)),
            Token::new(2, 1, TokenKind::NUM(23)),
            Token::new(4, 1, TokenKind::OP(Operation::Add)),
        ]
    );

    let mut source = "7+3\n-4".chars().peekable();
    let tokens = tokenize(&mut source).unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::new(1, 1, TokenKind::NUM(7)),
            Token::new(2, 1, TokenKind::OP(Operation::Add)),
            Token::new(3, 1, TokenKind::NUM(3)),
            Token::new(1, 2, TokenKind::OP(Operation::Sub)),
            Token::new(2, 2, TokenKind::NUM(4)),
        ]
    );
}

pub fn compile_from_source(source: String) -> Result<String> {
    let mut source = source.chars().peekable();

    let mut output = String::new();

    let mut token = tokenize(&mut source).unwrap().into_iter().peekable();

    writeln!(output, ".intel_syntax noprefix")?;
    writeln!(output, ".globl main")?;
    writeln!(output, "main:")?;

    writeln!(output, "  mov rax, {}", expect_number(&mut token)?)?;

    while token.peek().is_some() {
        if consume(&mut token, Operation::Add) {
            writeln!(output, "  add rax, {}", expect_number(&mut token)?)?;
            continue;
        }

        expect(&mut token, Operation::Sub)?;

        writeln!(output, "  sub rax, {}", expect_number(&mut token)?)?;
    }

    writeln!(output, "  ret")?;

    return Ok(output);
}
