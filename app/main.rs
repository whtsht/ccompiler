use ccompiler::gen::compile_from_source;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut source = Vec::new();
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        source.push(line.unwrap());
    }

    match compile_from_source(source) {
        Ok(dest) => println!("{}", dest),
        Err(err) => println!("{}", err),
    }
}
