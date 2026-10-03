use std::{env, fs, process::Command};
mod lexer;
use lexer::tokenize;
mod ast;
use ast::Stmt;
mod parser;
use parser::Parser;
mod vm;
use vm::VM;

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

    let mut final_stmts = Vec::new();
    for stmt in &stmts {
        if let Stmt::Include { lib_name } = stmt {
            let lib_source_code = fs::read_to_string(lib_name).expect(&format!("main: error reading file: {}", lib_name));
            let lib_tokens = tokenize(&lib_source_code);
            let mut lib_parser = Parser::new(lib_tokens);
            let lib_stmts = lib_parser.parse_tokens();
            final_stmts.extend(lib_stmts);
        }
    }
    final_stmts.extend(stmts);

    let c_code = codegen::generate_c_code(&final_stmts);

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

    // println!("\nVM:");

    // let mut vm = VM::new();
    // vm.execute(final_stmts);
}
