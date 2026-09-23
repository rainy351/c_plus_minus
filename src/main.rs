mod lexer;
use lexer::tokenize;

fn main() {
    let source_code = r#"
        // тест лексера C±
        include <std.xd>;
        idk a = 10;
        manual int b = 20;
        manual string msg = "Hello, C±!\n";
        F a;
    "#;

    println!("{:?}", tokenize(source_code));
}