mod lexer;
use lexer::tokenize;

mod ast;

mod parser;
use parser::Parser;

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

    let source_code = r#"
        include <std.xd>;
        idk my_variable = 4;
        F my_variable;
        manual int manual_var = 5;
        manual string manual_string = "asda";
        // manual int v2 = 2+4;
    "#;

    let tokens = tokenize(source_code);
    println!("\nTOKENS:");
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    
    println!("\nAST:");
    // let stmt1 = parser.parse_stmt();
    // println!("{:?}", stmt1);

    // let stmt2 = parser.parse_stmt();
    // println!("{:?}", stmt2);

    // let stmt3 = parser.parse_stmt();
    // println!("{:?}", stmt3);

    // let stmt4 = parser.parse_stmt();
    // println!("{:?}", stmt4);

    let stmts = parser.parse_tokens();

    for (i, stmt) in stmts.iter().enumerate() {
        println!("{}: {:?}", i + 1, stmt);
    }
}
