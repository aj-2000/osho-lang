use std::fs;

use crate::ast::{Parser, SemanticAnalyzer};
use crate::lexer::Lexer;

mod ast;
mod lexer;

fn main() {
    let file_path = "./examples/add-two-numbers.osho";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.get_tokens();

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse().expect("Failed to parse");
    println!("{:#?}", ast);
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Semantic analysis failed");
}
