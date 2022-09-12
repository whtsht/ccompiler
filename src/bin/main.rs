use ccompiler::compile;

fn main() {
    match compile() {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }
}
