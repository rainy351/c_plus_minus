#[derive(Debug)]
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
    Scope(usize),   // (: :) с подсчетом пробелов

    // 2. Литералы и идентификаторы с данными
    Ident(String),       // Имя переменной или функции
    Number(i64),         // Число
    StringLit(String),   // Строка

    // 3. Операторы и пунктуация
    Assign,       // =
    Plus,         // +
    Minus,        // -
    Mult,         // *

    Div,          // /

    Semicolon,    // ;
    Dot,          // .
    Comma,        // ,
    LParen,       // (
    RParen,       // )
    LBrace,       // {
    RBrace,       // }
    Gt,           // >
    Lt,           // <
    Output,       // ><

    // 4. На время разработки
    NotImplemented,
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut chars = source_code.chars().peekable();
    let mut tokens = Vec::new();
    // tokens.push(Token::Shrug);
    // tokens
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
            '=' => { tokens.push(Token::Assign); chars.next(); }
            ';' => { tokens.push(Token::Semicolon); chars.next(); }
            '.' => { tokens.push(Token::Dot); chars.next(); }
            '<' => { tokens.push(Token::Lt); chars.next(); }
            '>' => { tokens.push(Token::Gt); chars.next(); }
            
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
                // num_str.push(*ch);

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
            
            _ => { tokens.push(Token::NotImplemented); chars.next(); }
        }
    }

    return tokens
}
