use crate::{ast::{Expr, Stmt}, lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        return token;
    }

    pub fn expect_ident(&mut self) -> String {
        match self.advance() {
            Some(Token::Ident(name)) => {
                name.clone()
            },
            _ => {
                panic!("no ident???");
            }
        }
    }

    pub fn expect_libname(&mut self) -> String {
        match self.advance() {
            Some(Token::Lt) => {
                let mut lib_name = String::new();

                while let Some(token) = self.peek(){
                    match self.peek() {
                        Some(Token::Ident(libpart)) => {
                            lib_name += libpart;
                            self.advance();
                        },
                        Some(Token::Dot) => {
                            lib_name += ".";
                            self.advance();
                        },
                        Some(Token::Gt) => {
                            self.advance();
                            break;
                        },
                        _ => {
                            panic!("no libname??? (during parse)");
                        }
                    }
                }

                return lib_name;
            },
            _ => {
                panic!("no libname???");
            }
        }
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::PressF) => {
                self.advance();
                let var_name = self.expect_ident();
                self.advance();
                Stmt::FreeVar { var_name }
            },
            Some(Token::Include) => {
                self.advance();
                let lib_name = self.expect_libname();
                self.advance();
                Stmt::Include { lib_name }
            }
            _ => panic!("idk what to do :(")
        }
    }
}
