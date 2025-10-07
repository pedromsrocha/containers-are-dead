use anyhow::{Context, bail};
use fallible_iterator::FallibleIterator;
use serde::{Deserialize, Serialize};
use serde_querystring::ParseMode;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};
use std::str::Chars;
use std::str::FromStr;

// curl --request POST --data '10 / 2' http://localhost:3000/
// curl --request POST --data '10 / 2' http://localhost:3000?session=<ID>

#[derive(Deserialize)]
struct Params {
    session: Option<i64>,
}

#[derive(Serialize)]
struct EvalResponse {
    session: i64,
    result: Option<f64>,
}

/// A simple Spin HTTP component.
#[http_component]
pub fn handler(req: Request) -> anyhow::Result<impl IntoResponse> {
    let params: Params = serde_querystring::from_str(req.query(), ParseMode::UrlEncoded)?;

    let connection = Connection::open_default()?;

    let session_id = if let Some(session_id) = params.session {
        let ret = connection.execute(
            "SELECT 1 FROM sessions WHERE session_id == (?)",
            &[Value::Integer(session_id)],
        )?;
        let _ = ret.rows().next().context("session ID does not exist")?;

        session_id
    } else {
        establish_new_session(&connection)?
    };

    let input = str::from_utf8(req.body())?;
    let result = Evaluator::new(session_id, &connection, input).eval()?;

    let body = serde_json::to_vec(&EvalResponse {
        session: session_id,
        result,
    })?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(body)
        .build())
}

fn establish_new_session(db: &Connection) -> anyhow::Result<i64> {
    let ret = db.execute(
        "INSERT INTO sessions DEFAULT VALUES RETURNING session_id",
        &[],
    )?;

    let session_id = ret
        .rows()
        .map(|row| row.get("session_id"))
        .next()
        .unwrap()
        .unwrap();

    Ok(session_id)
}

#[derive(Clone)]
struct Evaluator<'a, 'db> {
    parser: fallible_iterator::Peekable<Parser<'a>>,
    session_id: i64,
    db: &'db Connection,
}

impl<'a, 'db> Evaluator<'a, 'db> {
    pub fn new(session_id: i64, db: &'db Connection, input: &'a str) -> Self {
        Self {
            parser: Parser::new(input).peekable(),
            session_id,
            db,
        }
    }

    fn get_variable(&self, name: &str) -> anyhow::Result<f64> {
        let rowset = self.db.execute(
            "SELECT key, value FROM variables WHERE session_id = (?) AND key = (?)",
            &[
                Value::Integer(self.session_id),
                Value::Text(name.to_string()),
            ],
        )?;

        let values: Vec<_> = rowset.rows().map(|row| row.get::<f64>("value")).collect();
        if values.is_empty() {
            bail!("no variable named {name}");
        }
        assert_eq!(values.len(), 1);

        Ok(values[0].expect("value was not a REAL. This is a bug!"))
    }

    fn set_variable(&self, name: &str, val: f64) -> anyhow::Result<()> {
        let execute_params = [
            Value::Integer(self.session_id),
            Value::Text(name.to_string()),
            Value::Real(val),
        ];

        self.db.execute(
            "INSERT INTO variables (session_id, key, value) VALUES (?, ?, ?)",
            execute_params.as_slice(),
        )?;

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
