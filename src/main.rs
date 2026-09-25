mod lexer;
use lexer::tokenize;

use std::{fs, env};

mod ast;

mod parser;
use parser::Parser;

mod vm;
use vm::VM;



fn main() {
    // let source_code = r#"
    //     // тест лексера C±
    //     include <std.xd>;
    //     idk a = 10;
    //     manual int b = 20;
    //     manual string msg = "Hello, C±!\n";
    //     F a;
    // "#;

    // let tokens = tokenize(source_code);

    // println!("{:?}", tokens);

    let args: Vec<String> = env::args().collect();
    let mut filename = "main.cpm";
    if args.len() > 1 { filename = args.get(1).unwrap(); }
    let source_code = fs::read_to_string(filename).expect("error reading file");

    let tokens = tokenize(&source_code);
    println!("\nTOKENS:");
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    
    // println!("\nAST:");
    // let stmt1 = parser.parse_stmt();
    // println!("{:?}", stmt1);

    // let stmt2 = parser.parse_stmt();
    // println!("{:?}", stmt2);

    // let stmt3 = parser.parse_stmt();
    // println!("{:?}", stmt3);

    // let stmt4 = parser.parse_stmt();
    // println!("{:?}", stmt4);

    println!("\nAST:");
    let stmts = parser.parse_tokens();

    for (i, stmt) in stmts.iter().enumerate() {
        println!("{}: {:?}", i + 1, stmt);
    }

    println!("\nVM:");

    let mut vm = VM::new();

    vm.execute(stmts);
}
