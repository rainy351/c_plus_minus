use std::{env, fs, process::Command};
mod lexer;
use lexer::tokenize;
mod ast;
use ast::Stmt;
mod parser;
use parser::Parser;
mod typechecker;
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
    let mut main_stmts = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Include { lib_name } => {
                let lib_source_code = fs::read_to_string(lib_name.clone()).unwrap_or_else(|_| panic!("main: error reading file: {}", lib_name));
                let lib_tokens = tokenize(&lib_source_code);
                let mut lib_parser = Parser::new(lib_tokens);
                let lib_stmts = lib_parser.parse_tokens();
                final_stmts.extend(lib_stmts);
            },
            _ => main_stmts.push(stmt),
        }
    }
    
    final_stmts.extend(main_stmts);

    let checker = typechecker::TypeChecker::new(final_stmts);
    let typed_ast = checker.typecheck();

    
    println!("\nTypized AST:");
    for (i, stmt) in typed_ast.iter().enumerate() {
        println!("{}: {:?}", i + 1, stmt);
    }

    let c_code = codegen::generate_c_code(typed_ast);

    println!("\nC:");
    println!("```\n");
    println!("{}", c_code);
    println!("```\n");

    let build_dir = "build";
    fs::create_dir_all(build_dir).expect("cannot create build directory");

    let runtime_dir = "runtime";
    let runtime_path = runtime_dir.to_string() + "/cpm_runtime.o";
    if std::path::Path::new(&runtime_path).is_file() {
        println!("runtime found");
    } else {
        // panic!("Рантайм не найден");
        let runtime_c = runtime_dir.to_string() + "/cpm_runtime.c";
        let runtime_o = runtime_dir.to_string() + "/cpm_runtime.o";
        
        let status = Command::new("gcc").args(["-c", &runtime_c, "-o", &runtime_o]).status();
        match status {
            Ok(s) if s.success() => println!("built cpm_runtime.o"),
            _ => panic!("failed to build Cpm runtime")
        }
    }
    let outc_path = build_dir.to_string() + "/out.c";
    let build_path = if cfg!(windows) {  build_dir.to_string() + "/out.exe" } else { build_dir.to_string() + "/out" };
    fs::write(&outc_path, &c_code).expect("failed to write out.c");
    let mut build_cmd = Command::new("gcc");
    build_cmd.arg("-Iruntime").args([&outc_path, &runtime_path, "-o", &build_path]);
    if cfg!(windows) {
        build_cmd.arg("-lbcrypt");
    }
    let status = build_cmd.status();
    match status {
        Ok(s) if s.success() => println!("built out"),
        _ => panic!("failed to build out.c")
    }
}
