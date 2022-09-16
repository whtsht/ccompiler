use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn assert_compiler(input: &str, expected: Option<i32>) {
    fs::write("./input", input).expect("failed to write the file");

    let handle = Command::new("./target/debug/ccompiler").output().unwrap();

    let mut file = File::create("./tmp.s").unwrap();
    file.write_all(&handle.stdout).unwrap();

    Command::new("cc")
        .args(["-o", "tmp", "tmp.s"])
        .output()
        .unwrap();

    let output = Command::new("./tmp").output().unwrap();
    assert_eq!(output.status.code(), expected, "{}", input);
}

#[test]
fn test_compiler() {
    assert_compiler("0", Some(0));
    assert_compiler("42", Some(42));
    assert_compiler("5+20-4", Some(21));
    assert_compiler("12 + 34 - 5", Some(41));
    assert_compiler("5+6*7", Some(47));
    assert_compiler("5*(9-6)", Some(15));
    assert_compiler("(3+5)/2", Some(4));
    assert_compiler("+10/(-2+7)", Some(2));
}
