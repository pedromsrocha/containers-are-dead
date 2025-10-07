use serde::Deserialize;
use std::process::Command;

// we can't use te spin testing framework here because that doesn't allow us to make POST requests with a body ://

#[derive(Debug, Deserialize)]
struct EvalResponse {
    session: i64,
    result: Option<f64>,
}

fn make_eval_request(input: &str, session: Option<i64>) -> EvalResponse {
    let url = if let Some(session) = session {
        &format!("http://127.0.0.1:3000/eval?session={session}")
    } else {
        "http://127.0.0.1:3000/eval"
    };
    let out = Command::new("curl")
        .args([
            "--fail-with-body",
            "--request",
            "POST",
            "--data",
            input,
            url,
        ])
        .output()
        .unwrap();

    assert!(out.status.success());

    println!("{:?}", String::from_utf8_lossy(&out.stdout));
    serde_json::from_slice(&out.stdout).unwrap()
}

fn make_delete_request(var: &str, session: Option<i64>) {
    let url = if let Some(session) = session {
        format!("http://127.0.0.1:3000/clear/{var}?session={session}")
    } else {
        format!("http://127.0.0.1:3000/clear/{var}")
    };
    let out = Command::new("curl")
        .args(["--fail-with-body", "--request", "DELETE", &url])
        .output()
        .unwrap();

    assert!(out.status.success());
}

#[test]
fn test_simple_arithmetic() {
    assert_eq!(make_eval_request("2 + 3", None).result.unwrap(), 5.0);
    assert_eq!(make_eval_request("10 - 4", None).result.unwrap(), 6.0);
    assert_eq!(make_eval_request("3 * 4", None).result.unwrap(), 12.0);
    assert_eq!(make_eval_request("15 / 3", None).result.unwrap(), 5.0);
}

#[test]
fn test_precedence() {
    assert_eq!(make_eval_request("2 + 3 * 4", None).result.unwrap(), 14.0);
    assert_eq!(make_eval_request("(2 + 3) * 4", None).result.unwrap(), 20.0);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(make_eval_request("-5 + 3", None).result.unwrap(), -2.0);
    assert_eq!(make_eval_request("10 + -5", None).result.unwrap(), 5.0);
}

#[test]
fn maaany_parens() {
    assert_eq!(
        make_eval_request("((((((((8))))))))", None).result.unwrap(),
        8.0
    );
}

#[test]
fn assign_same_session() {
    let res = make_eval_request("a = 10", None);
    assert!(res.result.is_none());
    let res2 = make_eval_request("b = 10 * (7 + a)", Some(res.session));
    assert_eq!(res.session, res2.session);
    assert!(res2.result.is_none());
    assert_eq!(
        make_eval_request("b", Some(res.session)).result.unwrap(),
        170.0
    );
}

#[test]
fn assign_different_session() {
    let res = make_eval_request("a = 10", None);
    assert!(res.result.is_none());

    let res2 = make_eval_request("a = 7", None);
    assert!(res2.result.is_none());

    let res = make_eval_request("b = 10 * (7 + a)", Some(res.session));
    let res2 = make_eval_request("b = 10 * (7 + a)", Some(res2.session));

    assert_eq!(
        make_eval_request("b", Some(res.session)).result.unwrap(),
        170.0
    );
    assert_eq!(
        make_eval_request("b", Some(res2.session)).result.unwrap(),
        140.0
    );
}

#[test]
fn delete() {
    let res = make_eval_request("a = 10", None);
    assert!(res.result.is_none());

    let res = make_eval_request("b = 11", Some(res.session));
    assert!(res.result.is_none());

    let res = make_eval_request("c = 12", Some(res.session));
    assert!(res.result.is_none());

    let res = make_eval_request("c = 13", Some(res.session));
    assert!(res.result.is_none());

    make_delete_request("c", Some(res.session))
}

#[test]
#[should_panic]
fn delete_no_session() {
    let res = make_eval_request("c = 13", None);
    assert!(res.result.is_none());

    make_delete_request("c", None);
}
