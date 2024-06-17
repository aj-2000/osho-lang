
use std::collections::HashMap;

use crate::parser::{ASTNode, BinaryOperator};

pub struct SemanticAnalyzer {
    variables: HashMap<String, f64>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, node: &ASTNode) -> Result<(), String> {
        match node {
            ASTNode::Program(statements) => {
                for stmt in statements {
                    self.analyze(stmt)?;
                }
            }
            ASTNode::LetDeclaration { name, value } => {
                if self.variables.contains_key(name) {
                    return Err(format!("Variable '{}' is already declared", name));
                }
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), val);
            }
            ASTNode::Assignment { name, value } => {
                if !self.variables.contains_key(name) {
                    return Err(format!("Variable '{}' is not declared", name));
                }
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), val);
            }
            ASTNode::Increment(name) => {
                if let Some(val) = self.variables.get_mut(name) {
                    *val += 1.0;
                } else {
                    return Err(format!("Variable '{}' is not declared", name));
                }
            }
            ASTNode::Decrement(name) => {
                if let Some(val) = self.variables.get_mut(name) {
                    *val -= 1.0;
                } else {
                    return Err(format!("Variable '{}' is not declared", name));
                }
            }
            ASTNode::Print(expr) => {
                let val = self.evaluate_expression(expr)?;
                println!("{}", val);
            }

            _ => return Err("Unexpected AST node".to_string()),
        }
        Ok(())
    }

    fn evaluate_expression(&self, expr: &ASTNode) -> Result<f64, String> {
        match expr {
            ASTNode::Number(num) => Ok(*num),
            ASTNode::Identifier(name) => {
                if let Some(val) = self.variables.get(name) {
                    Ok(*val)
                } else {
                    Err(format!("Variable '{}' is not declared", name))
                }
            }
            ASTNode::BinaryOp { left, op, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                match op {
                    BinaryOperator::Plus => Ok(left_val + right_val),
                    BinaryOperator::Minus => Ok(left_val - right_val),
                    BinaryOperator::Divide => Ok(left_val / right_val),
                    BinaryOperator::Multiply => Ok(left_val * right_val),
                }
            }
            _ => Err("Unexpected expression node".to_string()),
        }
    }
}

