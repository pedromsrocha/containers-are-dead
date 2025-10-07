use std::process::Command;
use std::str::FromStr;

fn call(input: &str) -> f64 {
    let out = Command::new("cargo")
        .args([
            "run",
            "-p",
            "components_contd",
            "--target",
            "wasm32-wasip2",
            "--",
            input,
        ])
        .output()
        .unwrap();

    assert!(out.status.success());

    f64::from_str(str::from_utf8(&out.stdout).unwrap().trim()).unwrap()
}

#[test]
fn test_simple_arithmetic() {
    assert_eq!(call("2 + 3"), 5.0);
    assert_eq!(call("10 - 4"), 6.0);
    assert_eq!(call("3 * 4"), 12.0);
    assert_eq!(call("15 / 3"), 5.0);
}

#[test]
fn test_precedence() {
    assert_eq!(call("2 + 3 * 4"), 14.0);
    assert_eq!(call("(2 + 3) * 4"), 20.0);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(call("-5 + 3"), -2.0);
    assert_eq!(call("10 + -5"), 5.0);
}

#[test]
fn maaany_parens() {
    assert_eq!(call("((((((((8))))))))"), 8.0);
}
