use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

struct Evaluator<'a, 'vars> {
    parser: Peekable<Parser<'a>>,
    variables: &'vars mut HashMap<Cow<'a, str>, f64>,
}

impl<'a, 'vars> Evaluator<'a, 'vars> {
    pub fn new(input: &'a str, variables: &'vars mut HashMap<Cow<'a, str>, f64>) -> Self {
        Self {
            parser: Parser::new(input).peekable(),
            variables,
        }
    }

    pub fn eval(mut self) -> f64 {
        if let Some(Token::Variable(var)) = self
            .parser
            .clone()
            .next_if(|t| matches!(t, Token::Variable(_)))
        {
            if self
                .parser
                .next_if(|t| matches!(t, Token::Assign))
                .is_some()
            {
                let result = self.eval_expression();

                self.variables.insert(Cow::Owned(var), result);
                return 0.0;
            }
        }

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
            Token::Variable(var) => self.variables[var.as_str()],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        let mut vars = HashMap::new();
        assert_eq!(Evaluator::new("2 + 3", &mut vars).eval(), 5.0);
        assert_eq!(Evaluator::new("10 - 4", &mut vars).eval(), 6.0);
        assert_eq!(Evaluator::new("3 * 4", &mut vars).eval(), 12.0);
        assert_eq!(Evaluator::new("15 / 3", &mut vars).eval(), 5.0);
    }

    #[test]
    fn test_precedence() {
        let mut vars = HashMap::new();
        assert_eq!(Evaluator::new("2 + 3 * 4", &mut vars).eval(), 14.0);
        assert_eq!(Evaluator::new("(2 + 3) * 4", &mut vars).eval(), 20.0);
    }

    #[test]
    fn test_negative_numbers() {
        let mut vars = HashMap::new();
        assert_eq!(Evaluator::new("-5 + 3", &mut vars).eval(), -2.0);
        assert_eq!(Evaluator::new("10 + -5", &mut vars).eval(), 5.0);
    }

    #[test]
    fn with_variables() {
        let mut vars = HashMap::new();
        vars.insert("a".into(), 42.0);
        assert_eq!(Evaluator::new("-5 + 3 * a", &mut vars).eval(), 121.0);
    }

    #[test]
    fn with_assign() {
        let mut vars = HashMap::new();
        vars.insert("a".into(), 42.0);
        Evaluator::new("b = a", &mut vars).eval();
        assert_eq!(vars["b"], 42.0);
    }

    #[test]
    fn with_variables_backtrack() {
        let mut vars = HashMap::new();
        vars.insert("a".into(), 42.0);
        assert_eq!(Evaluator::new("a * -5 + 3", &mut vars).eval(), -207.0);
    }
}
