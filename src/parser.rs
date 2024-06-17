use crate::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    LetDeclaration {
        name: String,
        value: Box<ASTNode>,
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    Increment(String),
    Decrement(String),
    Print(Box<ASTNode>),
    BinaryOp {
        left: Box<ASTNode>,
        op: BinaryOperator,
        right: Box<ASTNode>,
    },
    Number(f64),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide, // Add other operators as needed
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(ASTNode::Program(statements))
    }

    fn declaration(&mut self) -> Result<ASTNode, String> {
        if self.match_token(Kind::Let) {
            self.let_declaration()
        } else if self.match_token(Kind::Identifier) {
            self.statement()
        } else {
            self.statement()
        }
    }

    fn let_declaration(&mut self) -> Result<ASTNode, String> {
        let name = self
            .consume(Kind::Identifier, "Expected identifier after 'let'")?
            .clone();
        self.consume(Kind::EqualsTo, "Expected '=' after let declaration")?;
        let value = self.expression()?;
        Ok(ASTNode::LetDeclaration {
            name: self.token_to_string(&name)?,
            value: Box::new(value),
        })
    }

    fn statement(&mut self) -> Result<ASTNode, String> {
        if self.match_token(Kind::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<ASTNode, String> {
        let expr = self.expression()?;
        Ok(ASTNode::Print(Box::new(expr)))
    }

    fn expression_statement(&mut self) -> Result<ASTNode, String> {
        let expr = self.expression()?;
        if self.match_token(Kind::Increment) {
            if let ASTNode::Identifier(name) = expr {
                return Ok(ASTNode::Increment(name));
            }
            return Err("Expected identifier before '++'".to_string());
        } else if self.match_token(Kind::Minus) && self.match_token(Kind::Minus) {
            if let ASTNode::Identifier(name) = expr {
                return Ok(ASTNode::Decrement(name));
            }
            return Err("Expected identifier before '--'".to_string());
        }
        Ok(expr)
    }

    fn expression(&mut self) -> Result<ASTNode, String> {
        self.arithmetic()
    }

    fn arithmetic(&mut self) -> Result<ASTNode, String> {
        let mut node = self.primary()?;
        while let Some(operator) = {
            if self.match_token(Kind::Plus) {
                Some(BinaryOperator::Plus)
            } else if self.match_token(Kind::Minus) {
                Some(BinaryOperator::Minus)
            } else if self.match_token(Kind::Multiply) {
                Some(BinaryOperator::Multiply)
            } else if self.match_token(Kind::Divide) {
                Some(BinaryOperator::Divide)
            } else {
                None
            }
        } {
            let right = self.primary()?;
            node = ASTNode::BinaryOp {
                left: Box::new(node),
                op: operator,
                right: Box::new(right),
            };
        }
        Ok(node)
    }
    
    fn primary(&mut self) -> Result<ASTNode, String> {
        let prev_token: Token = self.previous().clone();
        if self.match_token(Kind::Number) {
            let value = self.previous().clone();
            if let TokenValue::Number(num) = value.value {
                return Ok(ASTNode::Number(num));
            }
            return Err("Expected number".to_string());
        }

        if self.match_token(Kind::Identifier) {
            let name = self.previous().clone();
            return Ok(ASTNode::Identifier(self.token_to_string(&name)?));
        }

        if self.match_token(Kind::EqualsTo) {
            let expr = self.expression()?;
            return Ok(ASTNode::Assignment {
                name: self.token_to_string(&prev_token)?,
                value: Box::new(expr),
            });
        }

        if self.match_token(Kind::OpenParen) {
            let expr = self.expression()?;
            dbg!(self.tokens[self.current + 1].clone());
            self.consume(Kind::CloseParen, "Expected ')' after expression")?;
            return Ok(expr);
        }

        if self.match_token(Kind::Increment) {
            return Ok(ASTNode::Increment(self.token_to_string(&prev_token)?));
        }

        if self.match_token(Kind::Decrement) {
            return Ok(ASTNode::Decrement(self.token_to_string(&prev_token)?));
        }

        Err("Expected expression".to_string())
    }

    fn consume(&mut self, kind: Kind, message: &str) -> Result<Token, String> {
        if self.check(kind) {
            return Ok(self.advance().clone());
        }
        Err(message.to_string())
    }

    fn match_token(&mut self, kind: Kind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: Kind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == Kind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn token_to_string(&self, token: &Token) -> Result<String, String> {
        if let TokenValue::String(atom) = &token.value {
            Ok(atom.to_string())
        } else {
            Err("Expected string".to_string())
        }
    }
}
