use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

// curl --request POST --data '10 / 2' http://localhost:3000/

/// A simple Spin HTTP component.
#[http_component]
pub fn handle_foo(req: Request) -> anyhow::Result<impl IntoResponse> {
    let store = spin_sdk::key_value::Store::open_default()?;

    let input = str::from_utf8(req.body())?;
    let result = Evaluator::new(input, &store).eval();

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(result.map(|val| val.to_string()).unwrap_or_default())
        .build())
}

#[derive(Clone)]
struct Evaluator<'a, 'store> {
    parser: Peekable<Parser<'a>>,
    store: &'store spin_sdk::key_value::Store,
}

impl<'a, 'store> Evaluator<'a, 'store> {
    pub fn new(input: &'a str, store: &'store spin_sdk::key_value::Store) -> Self {
        Self {
            parser: Parser::new(input).peekable(),
            store,
        }
    }

    fn get_variable(&self, name: &str) -> f64 {
        let val = self
            .store
            .get(name)
            .expect("failed to load variable")
            .expect("not such variable in the store");

        f64::from_le_bytes(val.try_into().unwrap())
    }

    fn set_variable(&self, name: &str, val: f64) {
        self.store.set(name, &val.to_le_bytes()).unwrap();
    }

    pub fn eval(mut self) -> Option<f64> {
        let mut clone = self.clone();
        if let Some(Token::Variable(var)) =
            clone.parser.next_if(|t| matches!(t, Token::Variable(_)))
        {
            if clone
                .parser
                .next_if(|t| matches!(t, Token::Assign))
                .is_some()
            {
                let result = clone.eval_expression();

                self.set_variable(&var, result);
                return None;
            }
        }

        Some(self.eval_expression())
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
            Token::Variable(var) => self.get_variable(&var),
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

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    Assign,
}

#[derive(Clone)]
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
            '=' => Token::Assign,
            c if c.is_numeric() => {
                let mut str = c.to_string();
                while let Some(ch) = self.chars.next_if(|c| c.is_numeric() || *c == '.') {
                    str.push(ch);
                }
                Token::Number(f64::from_str(&str).unwrap())
            }
            c if c.is_alphabetic() => {
                let mut str = c.to_string();
                while let Some(ch) = self
                    .chars
                    .next_if(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                {
                    str.push(ch);
                }
                Token::Variable(str)
            }
            _ => panic!("unexpected character: {ch}"),
        };

        Some(token)
    }
}
