use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn assert_compiler(input: &str, expected: Option<i32>) {
    let mut handle = Command::new("./target/debug/ccompiler")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    handle
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = handle.wait_with_output().unwrap();

    let mut file = File::create("./tmp.s").unwrap();
    file.write_all(&output.stdout).unwrap();

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
}
