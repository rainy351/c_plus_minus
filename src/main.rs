use std::{env, fs, process::Command};
mod lexer;
use lexer::tokenize;
mod ast;
mod parser;
use parser::Parser;
mod vm;
use vm::VM;

use crate::lexer::Token::Comma;

mod codegen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "main.cpm";
    if args.len() > 1 { filename = args.get(1).unwrap(); }
    let source_code = fs::read_to_string(filename).expect("error reading file");

    let tokens = tokenize(&source_code);
    println!("\nTOKENS:");
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);

    let stmts = parser.parse_tokens();
    println!("\nAST:");
    for (i, stmt) in stmts.iter().enumerate() {
        println!("{}: {:?}", i + 1, stmt);
    }

    let c_code = codegen::generate_c_code(&stmts);

    println!("\nC:");
    println!("```\n");
    println!("{}", c_code);
    println!("```\n");

    fs::write("out.c", &c_code).expect("failed to write out.c");
    let status = Command::new("gcc").args(["out.c", "-o", if cfg!(windows) { "out.exe" } else { "out" }]).status();
    match status {
        Ok(s) if s.success() => println!("built out.exe"),
        _ => panic!("failed to build out.c")
    }

    println!("\nVM:");

    let mut vm = VM::new();
    vm.execute(stmts);
}
