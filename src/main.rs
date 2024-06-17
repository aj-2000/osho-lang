
use std::fs::{read_to_string, File};
use std::io::Write;
use std::process::Command;

mod lexer;
mod parser;
mod semantic_analyzer;
mod code_generator;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use code_generator::CodeGenerator;

fn main() {
    let file_path = "./test.osho";

    let contents = read_to_string(file_path).expect("Should have been able to read the file");

    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.get_tokens();
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse().expect("Failed to parse");

    let mut analyzer = SemanticAnalyzer::new();
    print!("\nInterpreter output:\n",);
    analyzer.analyze(&ast).expect("Semantic analysis failed");

    let mut generator = CodeGenerator::new();
    let code = generator.generate(&ast).unwrap();

    // Write the generated code to a C file
    let mut file = File::create("output.c").unwrap();
    file.write_all(code.as_bytes()).unwrap();

    // Compile the C file to create an executable
    let status = Command::new("gcc")
        .arg("output.c")
        .arg("-o")
        .arg("output")
        .status()
        .expect("Failed to compile");

    if !status.success() {
        eprintln!("Compilation failed");
        std::process::exit(1);
    }

    // Run the executable and capture its output
    let output = Command::new("./output")
        .output()
        .expect("Failed to run the executable");

    if !output.status.success() {
        eprintln!("Execution failed");
        std::process::exit(1);
    }

    let stdout = std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
    println!("\nExecutable output:\n{}", stdout);
}
