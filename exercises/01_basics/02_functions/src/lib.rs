use std::iter::Peekable;
use std::slice;
use std::str::Chars;
use std::str::FromStr;

#[unsafe(no_mangle)]
unsafe extern "C" fn parse_expression<'a>(ptr: *const u8, len: usize) -> (*mut Token, usize) {
    let input = unsafe { slice::from_raw_parts(ptr, len) };
    let input = str::from_utf8(input).unwrap();

    let tokens = Parser::new(input).collect::<Box<_>>();
    let tokens = Box::into_raw(tokens);

    (tokens.cast(), tokens.len())
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            chars: raw.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.next()?;

        let token = match ch {
            ' ' | '\t' => {
                while self.chars.next_if(|c| *c == ' ' || *c == '\t').is_some() {}
                return self.next();
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            c if c.is_numeric() => {
                let mut str = c.to_string();
                while let Some(ch) = self.chars.next_if(|c| c.is_numeric() || *c == '.') {
                    str.push(ch);
                }
                Token::Number(f64::from_str(&str).unwrap())
            }
            _ => panic!("unexpected character: {ch}"),
        };

        Some(token)
    }
}
