use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    WhichIsA,
    Represents,
    Takes,
    It,
    Outputs,
    Thanks,
    And,
    Then,

    // Identifiers, Literals
    String(String),
    Ident(String),
    Method(String),
    DataType(String),
    Number(f64),

    // Punctuation
    Colon,
    Comma,
    Add,
    Sub,

    Dot,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.input.peek().cloned() {
            Some(c) => match c {
                'a'..='z' | 'A'..='Z' | '_' => Some(self.identifier()),
                '0'..='9' => Some(self.number()),
                '"' => Some(self.method()),
                '\'' => Some(self.string()),
                '.' => {
                    self.input.next();
                    Some(Token::new(TokenType::Dot, self.line))
                }
                ',' => {
                    self.input.next();
                    Some(Token::new(TokenType::Comma, self.line))
                }
                ':' => {
                    self.input.next();
                    Some(Token::new(TokenType::Colon, self.line))
                }
                _ => {
                    self.input.next();
                    None
                }
            },
            None => Some(Token::new(TokenType::Eof, self.line)),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                }
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let mut word = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                word.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        let token_type = match word.as_str() {
            "which" => {
                self.skip_whitespace();
                if self.peek_word() == "is" {
                    self.consume_word("is");
                    self.skip_whitespace();
                    if self.peek_word() == "a" {
                        self.consume_word("a");
                        TokenType::WhichIsA
                    } else {
                        TokenType::Ident(word.to_string())
                    }
                } else {
                    TokenType::Ident(word.to_string())
                }
            }
            "number" => TokenType::DataType(String::from("number")),
            "string" => TokenType::DataType(String::from("string")),
            "plus" => TokenType::Add,
            "minus" => TokenType::Sub,
            "represents" => TokenType::Represents,
            "takes" => TokenType::Takes,
            "it" => TokenType::It,
            "outputs" => TokenType::Outputs,
            "thanks" => TokenType::Thanks,
            "and" => TokenType::And,
            "then" => TokenType::Then,
            _ => TokenType::Ident(word),
        };

        Token::new(token_type, self.line)
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) {
                num_str.push(self.input.next().unwrap());
            } else if c == '.' {
                let mut peek_iter = self.input.clone();
                peek_iter.next();
                if peek_iter.peek().map_or(false, |next| next.is_digit(10)) {
                    num_str.push(self.input.next().unwrap());
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Token::new(
            TokenType::Number(num_str.parse().expect("Invalid number")),
            self.line,
        )
    }

    fn method(&mut self) -> Token {
        self.input.next();
        let mut value = String::new();
        while let Some(&c) = self.input.peek() {
            if c == '"' {
                break;
            }
            value.push(self.input.next().unwrap());
        }
        self.input.next();
        Token::new(TokenType::Method(value), self.line)
    }

    fn string(&mut self) -> Token {
        self.input.next();
        let mut value = String::new();
        while let Some(&c) = self.input.peek() {
            if c == '"' {
                break;
            }
            value.push(self.input.next().unwrap());
        }
        self.input.next();
        Token::new(TokenType::String(value), self.line)
    }

    fn peek_word(&mut self) -> String {
        let mut temp_iter = self.input.clone();
        let mut word = String::new();
        while let Some(&c) = temp_iter.peek() {
            if c.is_alphanumeric() {
                word.push(temp_iter.next().unwrap());
            } else {
                break;
            }
        }
        word
    }

    fn consume_word(&mut self, expected: &str) {
        self.skip_whitespace();
        for c in expected.chars() {
            assert_eq!(self.input.next(), Some(c));
        }
    }
}
