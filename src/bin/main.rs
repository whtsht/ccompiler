use ccompiler::compile_from_source;
use std::io::{self, Read};

fn main() {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();

    match compile_from_source(source) {
        Ok(dest) => println!("{}", dest),
        Err(err) => println!("{:?}", err),
    }
}
