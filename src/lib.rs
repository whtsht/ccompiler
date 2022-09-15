mod error;
mod node;
mod token;

use crate::node::Node;
use error::Result;
use node::{expr, NodeKind};
use std::fmt::Write;
use std::iter::{Iterator, Peekable};
use token::tokenize;

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

pub fn gen(node: &Box<Node>, output: &mut String) -> Result<()> {
    if let NodeKind::Num(num) = node.kind() {
        writeln!(output, "  push {}", num)?;
        return Ok(());
    }

    gen(node.lhs().unwrap(), output)?;
    gen(node.rhs().unwrap(), output)?;

    writeln!(output, "  pop rdi")?;
    writeln!(output, "  pop rax")?;

    match node.kind() {
        NodeKind::Add => writeln!(output, "  add rax, rdi")?,
        NodeKind::Sub => writeln!(output, "  sub rax, rdi")?,
        NodeKind::Mul => writeln!(output, "  imul rax, rdi")?,
        NodeKind::Div => {
            writeln!(output, "  cqo")?;
            writeln!(output, "  idiv rdi")?;
        }
        _ => (),
    }

    writeln!(output, "  push rax")?;

    Ok(())
}

pub fn compile_from_source(source: String) -> Result<String> {
    let source = source.chars().peekable();

    let mut output = String::new();

    let mut ts = tokenize(source)?;
    let node = expr(&mut ts)?;

    writeln!(output, ".intel_syntax noprefix")?;
    writeln!(output, ".globl main")?;
    writeln!(output, "main:")?;

    gen(&node, &mut output)?;

    writeln!(output, "  pop rax")?;
    writeln!(output, "  ret")?;

    return Ok(output);
}
