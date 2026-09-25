use std::{fs, env};
mod lexer;
use lexer::tokenize;
mod ast;
mod parser;
use parser::Parser;
mod vm;
use vm::VM;


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

    println!("\nVM:");

    let mut vm = VM::new();
    vm.execute(stmts);
}
