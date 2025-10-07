use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

// curl --request POST --data '10 / 2' http://localhost:3000/

/// A simple Spin HTTP component.
#[http_component]
pub fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    let input = str::from_utf8(req.body())?;

    let result = Evaluator::new(input).eval();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(result.to_string())
        .build())
}

struct Evaluator<'a> {
    parser: Peekable<Parser<'a>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            parser: Parser::new(input).peekable(),
        }
    }

    pub fn eval(mut self) -> f64 {
        self.eval_expression()
    }

    fn eval_expression(&mut self) -> f64 {
        let mut result = self.eval_term();

        while let Some(token) = self.parser.peek() {
            match token {
                Token::Plus => {
                    self.parser.next();
                    result += self.eval_term();
                }
                Token::Minus => {
                    self.parser.next();
                    result -= self.eval_term();
                }
                _ => break,
            }
        }

        result
    }

    fn eval_term(&mut self) -> f64 {
        let mut result = self.eval_factor();

        while let Some(token) = self.parser.peek() {
            match token {
                Token::Multiply => {
                    self.parser.next();
                    result *= self.eval_factor();
                }
                Token::Divide => {
                    self.parser.next();
                    result /= self.eval_factor();
                }
                _ => break,
            }
        }

        result
    }

    fn eval_factor(&mut self) -> f64 {
        match self.parser.next().expect("unexpected EOF") {
            Token::Number(n) => n,
            Token::LeftParen => {
                let result = self.eval_expression();
                match self.parser.next().expect("unexpected EOF") {
                    Token::RightParen => result,
                    _ => panic!("mismatched parentheses"),
                }
            }
            Token::Minus => -self.eval_factor(),
            t => panic!("mismatched token {t:?}"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
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
