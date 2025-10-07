// This module has *several* issues:
// 1. First we're missing a parser implementation!
//   This parser should accept input as a `&str` and parse the tokens into a `Vec`. See the `Token` enum below and the test cases for details.
// 2. The function declaration below is NOT valid! `&str` and `Vec<Token>` cannot be passed across an FFI boundary! You will need to fix this as well!
// If you get stuck, ask a neighbor or grab a trainer!
#[unsafe(no_mangle)]
unsafe extern "C" fn parse_expression<'a>(input: &str) -> Vec<Token> {
    todo!()
}

// The token that our parse should recognize
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Numbers: 1, or 4.5, or 100000000. Must fit within an f64.
    Number(f64),
    /// The `+` symbol
    Plus,
    /// The `-` symbol
    Minus,
    /// The `*` symbol
    Multiply,
    /// The `/` divide symbol
    Divide,
    /// The `(` symbol
    LeftParen,
    /// The `)` symbol
    RightParen,
}
