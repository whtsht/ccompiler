use crate::error::CResult;
use crate::error::CompileError;
use crate::node::program;
use crate::node::Node;
use crate::token::{tokenize, TokenKind};
use std::fmt::Write;

pub fn gen_lval(node: &Box<Node>, output: &mut String) -> CResult<()> {
    if let TokenKind::LocalVar { offset, .. } = node.kind() {
        writeln!(output, "  mov rax, rbp")?;
        writeln!(output, "  sub rax, {}", offset)?;
        writeln!(output, "  push rax")?;
    } else {
        return Err(CompileError::ParseError);
    }
    Ok(())
}

pub fn gen(node: &Box<Node>, output: &mut String) -> CResult<()> {
    match node.kind() {
        TokenKind::Num(num) => {
            writeln!(output, "  push {}", num)?;
            return Ok(());
        }
        TokenKind::LocalVar { .. } => {
            gen_lval(node, output)?;
            writeln!(output, "  pop rax")?;
            writeln!(output, "  mov rax, [rax]")?;
            writeln!(output, "  push rax")?;
            return Ok(());
        }
        TokenKind::Assign => {
            gen_lval(
                node.lhs()
                    .as_ref()
                    .ok_or_else(|| CompileError::ParseError)?,
                output,
            )?;
            gen(
                node.rhs()
                    .as_ref()
                    .ok_or_else(|| CompileError::ParseError)?,
                output,
            )?;
            writeln!(output, "  pop rdi")?;
            writeln!(output, "  pop rax")?;
            writeln!(output, "  mov [rax], rdi")?;
            writeln!(output, "  push rdi")?;
            return Ok(());
        }
        _ => {}
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
        TokenKind::Less => {
            writeln!(output, "  cmp rax, rdi")?;
            writeln!(output, "  setl al")?;
            writeln!(output, "  movzb rax, al")?;
        }
        TokenKind::LessOrEqual => {
            writeln!(output, "  cmp rax, rdi")?;
            writeln!(output, "  setle al")?;
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
    let program = program(&mut ts)?;

    writeln!(output, ".intel_syntax noprefix")?;
    writeln!(output, ".globl main")?;
    writeln!(output, "main:")?;

    writeln!(output, "  push rbp")?;
    writeln!(output, "  mov rbp, rsp")?;
    writeln!(output, "  sub rsp, 208")?; // = 26 * 8

    for node in program {
        gen(&node, &mut output)?;
        writeln!(output, "  pop rax")?;
    }

    writeln!(output, "  mov rsp, rbp")?;
    writeln!(output, "  pop rbp")?;
    writeln!(output, "  ret")?;

    return Ok(output);
}
