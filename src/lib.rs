mod error;
mod node;
mod token;

use error::Result;
use std::fmt::Write;
use std::iter::{Iterator, Peekable};
use token::{tokenize, Operation};

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

pub fn compile_from_source(source: String) -> Result<String> {
    let source = source.chars().peekable();

    let mut output = String::new();

    let mut ts = tokenize(source)?;

    writeln!(output, ".intel_syntax noprefix")?;
    writeln!(output, ".globl main")?;
    writeln!(output, "main:")?;

    writeln!(output, "  mov rax, {}", ts.expect_number()?)?;

    while ts.exist_next() {
        if ts.consume(Operation::Add) {
            writeln!(output, "  add rax, {}", ts.expect_number()?)?;
            continue;
        }

        ts.expect(Operation::Sub)?;

        writeln!(output, "  sub rax, {}", ts.expect_number()?)?;
    }

    writeln!(output, "  ret")?;

    return Ok(output);
}
