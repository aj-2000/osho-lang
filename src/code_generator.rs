use crate::parser::{ASTNode, BinaryOperator};

pub struct CodeGenerator {
    code: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    pub fn generate(&mut self, node: &ASTNode) -> Result<String, String> {
        self.code.clear();
        self.visit(node)?;
        let full_code = self.wrap_with_main(self.code.clone());
        Ok(full_code)
    }

    fn visit(&mut self, node: &ASTNode) -> Result<(), String> {
        match node {
            ASTNode::Program(statements) => {
                for stmt in statements {
                    self.visit(stmt)?;
                }
            }
            ASTNode::LetDeclaration { name, value } => {
                self.code.push_str(&format!("double {} = ", name));
                self.visit(value)?;
                self.code.push_str(";\n");
            }
            ASTNode::Assignment { name, value } => {
                self.code.push_str(&format!("{} = ", name));
                self.visit(value)?;
                self.code.push_str(";\n");
            }
            ASTNode::Increment(name) => {
                self.code.push_str(&format!("{}++;\n", name));
            }
            ASTNode::Decrement(name) => {
                self.code.push_str(&format!("{}--;\n", name));
            }
            ASTNode::Print(expr) => {
                self.code.push_str("printf(\"%f\\n\", ");
                self.visit(expr)?;
                self.code.push_str(");\n");
            }
            ASTNode::BinaryOp { left, op, right } => {
                self.code.push('(');
                self.visit(left)?;
                match op {
                    BinaryOperator::Plus => self.code.push_str(" + "),
                    BinaryOperator::Minus => self.code.push_str(" - "),
                    BinaryOperator::Multiply => self.code.push_str(" * "),
                    BinaryOperator::Divide => self.code.push_str(" / "),
                }
                self.visit(right)?;
                self.code.push(')');
            }
            ASTNode::Number(num) => {
                if num.fract() == 0.0 {
                    // If num is an integer, append ".0"
                    self.code.push_str(&format!("{:.1}", num));
                } else {
                    self.code.push_str(&num.to_string());
                }
            }
            ASTNode::Identifier(name) => {
                self.code.push_str(name);
            }
        }
        Ok(())
    }

    fn wrap_with_main(&self, code: String) -> String {
        format!(
            "#include <stdio.h>\n\nint main() {{\n{}\nreturn 0;\n}}",
            code
        )
    }
}
