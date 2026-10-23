#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 1. Ключевые слова C±
    Idk,            // idk | hz (динамический тип)
    Manual,         // manual (маркер строгого типа)
    TypeInt,        // int
    TypeString,     // string
    PressF,         // F (удаление переменной)
    If,             // if
    Else,           // else
    Stonks,         // stonks
    NotStonks,      // not_stonks
    While,          // while
    Func,           // func
    Return,         // return
    Include,        // include

    // это я чуть позже добавлю
    Shrug,          // ¯\_(ツ)_/¯
    KonamiCode,     // ↑↑↓↓←→←→BA
    // Scope(usize),   // (: :) с подсчетом пробелов

    // 2. Литералы и идентификаторы с данными
    Ident(String),       // Имя переменной или функции
    Number(i64),         // Число
    StringLit(String),   // Строка

    // 3. Операторы и пунктуация
    Assign,       // =
    Equals,       // ==
    NotEquals,    // !=
    Invert,       // !
    Plus,         // +
    Minus,        // -
    Mult,         // *
    Div,          // /
    Percent,      // %
    Semicolon,    // ;
    Colon,        // :
    Dot,          // .
    Comma,        // ,
    LParen,       // (
    RParen,       // )
    LBrace,       // {
    RBrace,       // }
    Gt,           // >
    Lt,           // <
    // Output,       // ><

    // 4. На время разработки
    NotImplemented,
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut chars = source_code.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' | '\n' => { chars.next(); },
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => { tokens.push(Token::Minus); chars.next(); }
            '*' => { tokens.push(Token::Mult); chars.next(); }
            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    while let Some(&next_ch) = chars.peek() {
                        chars.next();
                        if next_ch == '\n' {
                            break;
                        }
                    } 
                }
                else { tokens.push(Token::Div); }
            }
            '%' => { tokens.push(Token::Percent); chars.next(); }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::Equals);
                    chars.next();
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::NotEquals);
                    chars.next();
                } else {
                    tokens.push(Token::Invert);
                }
            }
            ';' => { tokens.push(Token::Semicolon); chars.next(); }
            ':' => { tokens.push(Token::Colon); chars.next(); }
            '.' => { tokens.push(Token::Dot); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '<' => { tokens.push(Token::Lt); chars.next(); }
            '>' => { tokens.push(Token::Gt); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            
            '"' => {
                let mut string_str = String::new();
                
                chars.next();

                while let Some(&next_ch) = chars.peek() {
                    if next_ch != '"' && next_ch != '\n' {
                        string_str.push(next_ch);
                        chars.next();
                    }
                    else {
                        chars.next();
                        break;
                    }
                }

                tokens.push(Token::StringLit(string_str));
            }

            '\'' => {
                let mut string_str = String::new();

                chars.next();

                while let Some(&next_ch) = chars.peek() {
                    if next_ch != '\'' && next_ch != '\n' {
                        string_str.push(next_ch);
                        chars.next();
                    }
                    else {
                        chars.next();
                        break;
                    }
                }

                tokens.push(Token::StringLit(string_str));
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident_str = String::new();
                
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_alphanumeric() || next_ch == '_' {
                        ident_str.push(next_ch);
                        chars.next();
                    }
                    else {
                        break;
                    }
                }

                match ident_str.as_str() {
                    "include" => tokens.push(Token::Include),
                    "idk" | "hz" => tokens.push(Token::Idk),
                    "manual" => tokens.push(Token::Manual),
                    "int" => tokens.push(Token::TypeInt),
                    "str" => tokens.push(Token::TypeString),
                    "string" => tokens.push(Token::TypeString),
                    "F" => tokens.push(Token::PressF),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "stonks" => tokens.push(Token::Stonks),
                    "not_stonks" => tokens.push(Token::NotStonks),
                    "while" => tokens.push(Token::While),
                    "func" => tokens.push(Token::Func),
                    "return" => tokens.push(Token::Return),
                    _ => tokens.push(Token::Ident(ident_str)),
                }
            }
            
            '0'..='9' => {
                let mut num_str = String::new();

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        num_str.push(next_ch);
                        chars.next();
                    }
                    else {
                        break;
                    }
                }
                tokens.push(Token::Number(num_str.parse().expect("couldn't parse numbers string during lexing")));
            }

            '¯' => {
                let shrug = "¯\\_(ツ)_/¯";
                let shrug_len = shrug.chars().count();
                if chars.clone().take(shrug_len).eq(shrug.chars()) {
                    chars.nth(shrug_len - 1);
                    tokens.push(Token::Shrug);
                }
                else {
                    panic!("unknown char at: `¯`. maybe you meant: `{shrug}`?")
                }
            }

            '↑' => {
                let konami_code = "↑↑↓↓←→←→BA";
                let konami_code_len = konami_code.chars().count();
                if chars.clone().take(konami_code_len).eq(konami_code.chars()) {
                    chars.nth(konami_code_len - 1);
                    tokens.push(Token::KonamiCode);
                }
                else {
                    panic!("unknown char at: `↑`. maybe you meant: `{konami_code}`?")
                }
            }
            
            _ => { tokens.push(Token::NotImplemented); chars.next(); }
        }
    }

    tokens
}
