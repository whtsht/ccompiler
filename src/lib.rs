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

pub fn compile_from_source(source: String) -> Result<String> {
    let mut source = source.chars().peekable();

    let mut output = String::new();

    writeln!(output, ".intel_syntax noprefix")?;
    writeln!(output, ".globl main")?;
    writeln!(output, "main:")?;
    writeln!(output, "  mov rax, {}", to_num(&mut source).unwrap())?;

    while let Some(s) = source.next() {
        if s == '+' {
            writeln!(output, "  add rax, {}", to_num(&mut source).unwrap())?;
            continue;
        }
        if s == '-' {
            writeln!(output, "  sub rax, {}", to_num(&mut source).unwrap())?;
            continue;
        }
        if s == '\n' {
            break;
        }
        return Err(CompileError::Unexpected);
    }

    writeln!(output, "  ret")?;

    return Ok(output);
}
