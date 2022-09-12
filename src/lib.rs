use std::env;
use std::fmt::Write;
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

pub fn compile() -> Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(CompileError::InvalidArgs);
    }

    let mut output = String::new();

    writeln!(output, ".intel_syntax noprefix")?;
    writeln!(output, ".globl main")?;
    writeln!(output, "main:")?;
    writeln!(output, "  mov rax, {}", args[1].parse::<u8>()?)?;
    writeln!(output, "  ret")?;

    return Ok(output);
}
