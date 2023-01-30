use tan::error::Error;
use tan::{lexer::token::Token, parser::NonRecoverableError, range::Ranged};

// #TODO rename to `formatter.rs`
// #TODO how to handle parse errors?

const IDENT_SIZE: usize = 4;

pub struct Formatter<I>
where
    I: IntoIterator<Item = Ranged<Token>>,
{
    tokens: I::IntoIter,
    nesting: usize,
    errors: Vec<Error>,
}

impl<I> Formatter<I>
where
    I: IntoIterator<Item = Ranged<Token>>,
{
    pub fn new(tokens: I) -> Self {
        let tokens = tokens.into_iter();

        Self {
            tokens,
            nesting: 0,
            errors: Vec::new(),
        }
    }

    fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn format_list(&mut self, delimiter: Token) -> Result<String, NonRecoverableError> {
        let mut output = String::new();

        self.nesting += 1;

        loop {
            let Some(token) = self.tokens.next() else {
                // #TODO how to handle this?
                self.push_error(Error::UnterminatedList);
                return Err(NonRecoverableError {});
            };

            if token.0 == delimiter {
                self.nesting -= 1;
                return Ok(output);
            } else {
                // #TODO set correct range
                let s = self.format_expr(token)?;
                output.push_str(&format!("{}{s}\n", " ".repeat(self.nesting * IDENT_SIZE)));
            }
        }
    }

    pub fn format_expr(&mut self, token: Ranged<Token>) -> Result<String, NonRecoverableError> {
        let Ranged(t, _) = token;

        let output = match t {
            Token::Comment(s) => s,
            Token::String(s) => format!("\"{s}\""),
            Token::Symbol(s) => s,
            Token::Int(n) => n.to_string(),
            Token::Float(n) => n.to_string(),
            Token::Annotation(s) => format!("#{s}"),
            Token::Quote => "'".to_owned(),
            Token::LeftParen => {
                let s = self.format_list(Token::RightParen)?;
                format!("({s})")
            }
            Token::LeftBracket => {
                // Syntactic sugar for a List/Array.

                let s = self.format_list(Token::RightBracket)?;
                format!("[\n{s}]")
            }
            Token::LeftBrace => {
                // Syntactic sugar for a Dict.

                let s = self.format_list(Token::RightBrace)?;
                format!("{{\n{s}}})")
            }
            Token::RightParen | Token::RightBracket | Token::RightBrace => {
                // #TODO custom error for this?
                self.push_error(Error::UnexpectedToken(t));
                // Parsing can continue.
                return Ok("".to_owned());
            }
        };

        Ok(output)
    }

    /// Formats an expression in aestheticall pleasing form.
    /// This is the standard textual representation of expressions.
    pub fn format(&mut self) -> Result<String, Vec<Error>> {
        let mut output = String::new();

        loop {
            let Some(token) = self.tokens.next() else {
                break;
            };

            let Ok(s) = self.format_expr(token) else {
                // A non-recoverable parse error was detected, stop parsing.
                let errors = std::mem::take(&mut self.errors);
                return Err(errors);
                // break;
            };

            output.push_str(&s);
        }

        Ok(output)
    }
}
