use std::fmt::Write;
use std::iter::{Iterator, Peekable};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("The number of arguments is incorrect")]
    InvalidArgs,
    #[error("Parse Error: number")]
    Parse(ParseIntError),
    #[error("Format Error")]
    FmtError(std::fmt::Error),
    #[error("Unexpected input")]
    Unexpected,
}

impl From<ParseIntError> for CompileError {
    fn from(err: ParseIntError) -> Self {
        CompileError::Parse(err)
    }
}

impl From<std::fmt::Error> for CompileError {
    fn from(err: std::fmt::Error) -> Self {
        CompileError::FmtError(err)
    }
}

pub type Result<T> = std::result::Result<T, CompileError>;

pub fn to_num<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<usize> {
    let mut result = iter.next()?.to_digit(10)? as usize;
    loop {
        match iter.peek() {
            Some(c) => match c.to_digit(10) {
                Some(i) => {
                    result = result * 10 + i as usize;
                    iter.next();
                }
                None => break,
            },
            None => break,
        }
    }
    Some(result)
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
enum Token {
    OP(Operation),
    NUM(usize),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Sub,
}

impl From<usize> for Token {
    fn from(num: usize) -> Self {
        Token::NUM(num)
    }
}

impl From<Operation> for Token {
    fn from(op: Operation) -> Self {
        Token::OP(op)
    }
}

fn tokenize<I: Iterator<Item = char>>(source: &mut Peekable<I>) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    while let Some(&s) = source.peek() {
        match s {
            '+' => {
                tokens.push(Operation::Add.into());
                source.next();
            }
            '-' => {
                tokens.push(Operation::Sub.into());
                source.next();
            }
            '\n' | ' ' => {
                source.next();
            }
            _ => {
                if let Some(num) = to_num(source) {
                    tokens.push(num.into());
                } else {
                    return Err(CompileError::Unexpected);
                }
            }
        }
    }

    Ok(tokens)
}

fn expect<I: Iterator<Item = Token>>(token: &mut I, expected: Operation) -> Result<Operation> {
    if let Some(t) = token.next() {
        match t {
            Token::OP(operation) if operation == expected => return Ok(operation),
            _ => return Err(CompileError::Unexpected),
        }
    }
    Err(CompileError::Unexpected)
}

fn expect_number<I: Iterator<Item = Token>>(token: &mut I) -> Result<usize> {
    if let Some(Token::NUM(num)) = token.next() {
        return Ok(num);
    } else {
        return Err(CompileError::Unexpected);
    }
}

fn consume<I: Iterator<Item = Token>>(token: &mut Peekable<I>, expected: Operation) -> bool {
    if let Some(t) = token.peek() {
        match t {
            Token::OP(operation) if operation == &expected => {
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
        vec![Token::NUM(1), Token::OP(Operation::Add), Token::NUM(4)]
    );

    let mut source = "-23".chars().peekable();
    let tokens = tokenize(&mut source).unwrap();
    assert_eq!(tokens, vec![Token::OP(Operation::Sub), Token::NUM(23)]);
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
        }

        expect(&mut token, Operation::Sub)?;

        writeln!(output, "  sub rax, {}", expect_number(&mut token)?)?;
    }

    writeln!(output, "  ret")?;

    return Ok(output);
}
