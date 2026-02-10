use crate::lexer::{Token, TokenType};

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Number(f64),
    String(String),
    Ident(String),
    Method(String),
    VarDecl {
        name: Box<AstNode>,
        data_type: Box<AstNode>,
        value: Box<AstNode>,
    },
    BinOp {
        left: Box<AstNode>,
        op: TokenType,
        right: Box<AstNode>,
    },
    MethodDef {
        name: Box<AstNode>,
        args: Vec<AstNode>,
        body: Vec<AstNode>,
    },
    MethodCall {
        name: Box<AstNode>,
        args: Vec<AstNode>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut nodes = Vec::new();
        while self.peek().token_type != TokenType::Eof {
            if let Some(node) = self.parse_statement() {
                nodes.push(node);
            }
        }
        nodes
    }

    fn parse_statement(&mut self) -> Option<AstNode> {
        let token = self.peek();
        match token.token_type {
            TokenType::Ident(_) => self.parse_var_decl(),
            TokenType::Method(_) => {
                if self.peek_next().token_type == TokenType::Takes {
                    self.parse_method_def()
                } else {
                    self.parse_chained_call()
                }
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_var_decl(&mut self) -> Option<AstNode> {
        let name = self.parse_primary()?;
        self.expect(TokenType::WhichIsA)?;
        let data_type = self.parse_primary()?;
        self.expect(TokenType::Represents)?;
        let value = self.parse_primary()?;
        self.expect(TokenType::Dot)?;

        Some(AstNode::VarDecl {
            name: Box::new(name),
            data_type: Box::new(data_type),
            value: Box::new(value),
        })
    }

    fn parse_method_def(&mut self) -> Option<AstNode> {
        let name = self.parse_primary()?;
        self.expect(TokenType::Takes)?;
        let args = self.parse_def_args()?;
        self.expect(TokenType::Colon)?;
        let body = self.parse_body();
        if body.is_none() {
            return None;
        }
        self.expect(TokenType::Thanks)?;

        let node = AstNode::MethodDef {
            name: Box::new(name),
            args,
            body: body.unwrap(),
        };
        Some(node)
    }

    fn parse_chained_call(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_method_call()?;

        while self.peek().token_type == TokenType::Then {
            self.advance();

            let name = self.parse_primary()?;
            let mut args = Vec::new();
            let mut found_it = false;

            while self.peek().token_type != TokenType::Dot
                && self.peek().token_type != TokenType::Then
            {
                if self.peek().token_type == TokenType::It {
                    self.advance();
                    args.push(expr.clone());
                    found_it = true;
                } else {
                    args.push(self.parse_primary()?);
                }
            }

            if !found_it {
                args.insert(0, expr);
            }

            expr = AstNode::MethodCall {
                name: Box::new(name),
                args,
            };
        }

        self.expect(TokenType::Dot)?;
        Some(expr)
    }

    fn parse_bin_op(&mut self) -> Option<AstNode> {
        let left = self.parse_primary()?;
        let op = self.peek().token_type.clone();
        self.advance();
        let right = self.parse_primary()?;

        Some(AstNode::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn parse_primary(&mut self) -> Option<AstNode> {
        let token = self.peek().clone();
        self.advance();
        match token.token_type {
            TokenType::Number(n) => Some(AstNode::Number(n)),
            TokenType::String(s) => Some(AstNode::String(s)),
            TokenType::Ident(id) => Some(AstNode::Ident(id)),
            TokenType::Method(m) => Some(AstNode::Method(m)),
            TokenType::DataType(d) => Some(AstNode::Ident(d)),
            TokenType::It => Some(AstNode::Ident("it".to_string())),
            _ => None,
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Option<()> {
        if self.peek().token_type == token_type {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek_next(&self) -> &Token {
        &self.tokens[self.pos + 1]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }
}
