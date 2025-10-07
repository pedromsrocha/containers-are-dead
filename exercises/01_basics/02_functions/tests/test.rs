extern crate functions;

use std::ffi::c_void;

use functions::Token;

fn parse_expression(str: &str) -> Vec<Token> {
    unsafe extern "C" {
        #[link_name = "parse_expression"]
        fn parse_expression_inner(
            in_ptr: *const u8,
            in_len: usize,
            out_ptr: *mut c_void,
            out_len: usize,
        ) -> usize;
    }

    let mut out: Vec<Token> = Vec::with_capacity(1024);

    let tokens_emitted = unsafe {
        parse_expression_inner(
            str.as_ptr(),
            str.len(),
            out.as_mut_ptr().cast(),
            out.capacity(),
        )
    };

    unsafe { out.set_len(tokens_emitted) };

    out
}

#[test]
fn basic_arithmetic_works() {
    assert_eq!(
        *parse_expression("2 + 3"),
        [Token::Number(2.0), Token::Plus, Token::Number(3.0)]
    );
    assert_eq!(
        *parse_expression("10 - 4"),
        [Token::Number(10.0), Token::Minus, Token::Number(4.0)]
    );
    assert_eq!(
        *parse_expression("3 * 4"),
        [Token::Number(3.0), Token::Multiply, Token::Number(4.0)]
    );
    assert_eq!(
        *parse_expression("15.4 / 3"),
        [Token::Number(15.4), Token::Divide, Token::Number(3.0)]
    );
}

#[test]
fn negative_numbers() {
    assert_eq!(
        *parse_expression("-5 + 3"),
        [
            Token::Minus,
            Token::Number(5.0),
            Token::Plus,
            Token::Number(3.0)
        ]
    );
    assert_eq!(
        *parse_expression("10 + -3"),
        [
            Token::Number(10.0),
            Token::Plus,
            Token::Minus,
            Token::Number(3.0)
        ]
    );
}

#[test]
fn parens() {
    assert_eq!(
        *parse_expression("(10 + 4) * 9"),
        [
            Token::LeftParen,
            Token::Number(10.0),
            Token::Plus,
            Token::Number(4.0),
            Token::RightParen,
            Token::Multiply,
            Token::Number(9.0)
        ]
    );
}

#[test]
fn maaany_parens() {
    assert_eq!(
        *parse_expression("(((((((()))))"),
        [
            Token::LeftParen,
            Token::LeftParen,
            Token::LeftParen,
            Token::LeftParen,
            Token::LeftParen,
            Token::LeftParen,
            Token::LeftParen,
            Token::LeftParen,
            Token::RightParen,
            Token::RightParen,
            Token::RightParen,
            Token::RightParen,
            Token::RightParen,
        ]
    );
}
