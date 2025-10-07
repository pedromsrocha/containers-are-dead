use anyhow::{Context, bail};
use fallible_iterator::FallibleIterator;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::str::Chars;
use std::str::FromStr;

// curl --request POST --data '10 / 2' http://localhost:3000/

/// A simple Spin HTTP component.
#[http_component]
pub fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    let store = spin_sdk::key_value::Store::open_default()?;

    let input = str::from_utf8(req.body())?;
    let result = Evaluator::new(input, &store).eval()?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(result.map(|val| val.to_string()).unwrap_or_default())
        .build())
}

#[derive(Clone)]
struct Evaluator<'a, 'store> {
    parser: fallible_iterator::Peekable<Parser<'a>>,
    store: &'store spin_sdk::key_value::Store,
}

impl<'a, 'store> Evaluator<'a, 'store> {
    pub fn new(input: &'a str, store: &'store spin_sdk::key_value::Store) -> Self {
        Self {
            parser: Parser::new(input).peekable(),
            store,
        }
    }

    fn get_variable(&self, name: &str) -> anyhow::Result<f64> {
        let val = self
            .store
            .get(name)?
            .context("not such variable in the store")?;

        Ok(f64::from_le_bytes(val.try_into().unwrap()))
    }

    fn set_variable(&self, name: &str, val: f64) -> anyhow::Result<()> {
        self.store.set(name, &val.to_le_bytes())?;
        Ok(())
    }

    pub fn eval(mut self) -> anyhow::Result<Option<f64>> {
        let mut clone = self.clone();
        if let Some(Token::Variable(var)) =
            clone.parser.next_if(|t| matches!(t, Token::Variable(_)))?
        {
            if clone
                .parser
                .next_if(|t| matches!(t, Token::Assign))?
                .is_some()
            {
                let result = clone.eval_expression()?;

                self.set_variable(&var, result)?;
                return Ok(None);
            }
        }

        self.eval_expression().map(Some)
    }

    fn eval_expression(&mut self) -> anyhow::Result<f64> {
        let mut result = self.eval_term()?;

        while let Some(token) = self.parser.peek()? {
            match token {
                Token::Plus => {
                    self.parser.next()?;
                    result += self.eval_term()?;
                }
                Token::Minus => {
                    self.parser.next()?;
                    result -= self.eval_term()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn eval_term(&mut self) -> anyhow::Result<f64> {
        let mut result = self.eval_factor()?;

        while let Some(token) = self.parser.peek()? {
            match token {
                Token::Multiply => {
                    self.parser.next()?;
                    result *= self.eval_factor()?;
                }
                Token::Divide => {
                    self.parser.next()?;
                    result /= self.eval_factor()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn eval_factor(&mut self) -> anyhow::Result<f64> {
        match self.parser.next()?.context("unexpected EOF")? {
            Token::Number(n) => Ok(n),
            Token::Variable(var) => self.get_variable(&var),
            Token::LeftParen => {
                let result = self.eval_expression()?;
                match self.parser.next()?.context("unexpected EOF")? {
                    Token::RightParen => Ok(result),
                    _ => bail!("mismatched parentheses"),
                }
            }
            Token::Minus => Ok(-self.eval_factor()?),
            t => bail!("mismatched token {t:?}"),
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
    chars: std::iter::Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(raw: &'a str) -> Self {
        Self {
            chars: raw.chars().peekable(),
        }
    }
}

impl<'a> FallibleIterator for Parser<'a> {
    type Item = Token;
    type Error = anyhow::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        let Some(ch) = self.chars.next() else {
            return Ok(None);
        };

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
                Token::Number(f64::from_str(&str)?)
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
            _ => bail!("unexpected character: {ch}"),
        };

        Ok(Some(token))
    }
}
