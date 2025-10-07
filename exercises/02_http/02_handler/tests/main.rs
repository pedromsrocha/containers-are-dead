use std::{process::Command, str::FromStr};

// we can't use te spin testing framework here because that doesn't allow us to make POST requests with a body ://

fn make_request(input: &str) -> f64 {
    let out = Command::new("curl")
        .args([
            "--fail-with-body",
            "--request",
            "POST",
            "--data",
            input,
            "http://127.0.0.1:3000",
        ])
        .output()
        .unwrap();

    assert!(out.status.success());

    f64::from_str(str::from_utf8(&out.stdout).unwrap().trim()).unwrap()
}

#[test]
fn test_simple_arithmetic() {
    assert_eq!(make_request("2 + 3"), 5.0);
    assert_eq!(make_request("10 - 4"), 6.0);
    assert_eq!(make_request("3 * 4"), 12.0);
    assert_eq!(make_request("15 / 3"), 5.0);
}

#[test]
fn test_precedence() {
    assert_eq!(make_request("2 + 3 * 4"), 14.0);
    assert_eq!(make_request("(2 + 3) * 4"), 20.0);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(make_request("-5 + 3"), -2.0);
    assert_eq!(make_request("10 + -5"), 5.0);
}

#[test]
fn maaany_parens() {
    assert_eq!(make_request("((((((((8))))))))"), 8.0);
}
