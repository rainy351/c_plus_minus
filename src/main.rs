mod lexer;
use lexer::tokenize;

mod ast;

fn main() {
    let source_code = r#"
        // тест лексера C±
        include <std.xd>;
        idk a = 10;
        manual int b = 20;
        manual string msg = "Hello, C±!\n";
        F a;
    "#;

    let tokens = tokenize(source_code);

    println!("{:?}", tokens);
}