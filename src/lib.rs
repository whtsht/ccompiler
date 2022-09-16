mod error;
mod node;
mod token;

use crate::node::Node;
use error::CResult;
use node::expr;
use std::fmt::Write;
use token::{tokenize, TokenKind};

pub fn gen(node: &Box<Node>, output: &mut String) -> CResult<()> {
    if let TokenKind::Num(num) = node.kind() {
        writeln!(output, "  push {}", num)?;
        return Ok(());
    }

    gen(node.lhs().unwrap(), output)?;
    gen(node.rhs().unwrap(), output)?;

    writeln!(output, "  pop rdi")?;
    writeln!(output, "  pop rax")?;

    match node.kind() {
        TokenKind::Add => writeln!(output, "  add rax, rdi")?,
        TokenKind::Sub => writeln!(output, "  sub rax, rdi")?,
        TokenKind::Mul => writeln!(output, "  imul rax, rdi")?,
        TokenKind::Div => {
            writeln!(output, "  cqo")?;
            writeln!(output, "  idiv rdi")?;
        }
        TokenKind::Equal => {
            writeln!(output, "  cmp rax, rdi")?;
            writeln!(output, "  sete al")?;
            writeln!(output, "  movzb rax, al")?;
        }
        TokenKind::NEqual => {
            writeln!(output, "  cmp rax, rdi")?;
            writeln!(output, "  setne al")?;
            writeln!(output, "  movzb rax, al")?;
        }
        _ => (),
    }

    writeln!(output, "  push rax")?;

    Ok(())
}

pub fn compile_from_source(source: Vec<String>) -> CResult<String> {
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
