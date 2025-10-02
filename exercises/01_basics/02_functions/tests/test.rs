extern crate functions;

use std::ptr;

use functions::Token;

fn parse_expression(str: &str) -> Box<[Token]> {
    unsafe extern "C" {
        #[link_name = "parse_expression"]
        fn parse_expression_inner(ptr: *const u8, len: usize) -> (*mut Token, usize);
    }

    let (ptr, len) = unsafe { parse_expression_inner(str.as_ptr(), str.len()) };

    unsafe { Box::from_raw(ptr::slice_from_raw_parts_mut(ptr, len)) }
}

#[test]
fn verify_it_works() {
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
