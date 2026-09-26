use core::panic;

use crate::{ast::{Expr, Stmt, VarType}, lexer::Token};
use crate::ast::Ops;

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

    pub fn expect_type(&mut self) -> VarType {
        match self.advance() {
            Some(Token::TypeInt) => {
                VarType::TypeInt
            },
            Some(Token::TypeString) => {
                VarType::TypeString
            },
            _ => {
                panic!("no ident???");
            }
        }
    }

    pub fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Some(Token::Number(num)) => {
                Expr::Number(*num)
            },
            Some(Token::StringLit(string)) => {
                Expr::StringLit(string.clone())
            },
            Some(Token::Ident(ident)) => {
                let name = ident.clone();
                // эта дрянь поддерживает 1 висячую запятую в конце
                if self.peek() == Some(&Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while self.peek().is_some() && self.peek() != Some(&Token::RParen) {
                        args.push(self.parse_expr());
                        if self.peek() == Some(&Token::Comma){
                            self.advance();
                        } else if self.peek() != Some(&Token::RParen) {
                            panic!("parser: expected `,` or `)`. FOUND={:?}", self.peek().unwrap())
                        }
                    }
                    self.advance();
                    Expr::Call { name: name, args: args }
                }
                else { Expr::VarIdent(name) }
            },
            other => {
                panic!("couldn't parse as expression????? GOT:{:?}", other.unwrap());
            }
        }
    }

    pub fn expect_libname(&mut self) -> String {
        match self.advance() {
            Some(Token::Lt) => {
                let mut lib_name = String::new();

                while let Some(_token) = self.peek(){
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
                self.advance(); // сам токен команды
                let var_name = self.expect_ident();
                self.advance(); // точка с запятой
                Stmt::PressF { var_name }
            },
            Some(Token::Include) => {
                self.advance();
                let lib_name = self.expect_libname();
                self.advance();
                Stmt::Include { lib_name }
            },
            Some(Token::Idk) => {
                self.advance();
                let var_name = self.expect_ident();

                if self.peek() == Some(&Token::Assign) {
                        self.advance();
                } else {
                    panic!("no assign????")
                }

                let var_value = self.parse_expr();

                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                } else {
                    panic!("no semicolon???? GOT:{:?}", self.peek());
                }
                
                Stmt::Variable { var_name, var_type: VarType::Idk, var_value }
            },
            Some(Token::Manual) => {
                self.advance();

                let var_type = self.expect_type();
                let var_name = self.expect_ident();

                if self.peek() == Some(&Token::Assign) {
                        self.advance();
                } else {
                    panic!("no assign????")
                }

                let var_value = self.parse_expr();

                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                } else {
                    panic!("no semicolon????");
                }
                
                Stmt::Variable { var_name, var_type, var_value }
            },
            Some(Token::Ident(_)) | Some(Token::Number(_)) | Some(Token::StringLit(_)) => {
                let expr = self.parse_expr();
                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                    Stmt::Expr(expr)
                }
                else {
                    panic!("parser: expected a semicolon `;` after expression. GOT:{:?}", self.peek())
                }
            }
            _ => panic!("parser: idk what to do ({:?}) :(", self.peek().unwrap())
        }
    }

    pub fn parse_expr_deep(&mut self) -> Expr {
        let mut left = self.parse_primary();
        
        while let Some(_token) = self.peek(){
            let op = match self.peek() {
                Some(Token::Mult) => Ops::Mult,
                Some(Token::Div) => Ops::Div,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_primary();
            left = Expr::Operation { a: Box::new(left), op: op, b: Box::new(right) };
        }
        return left;
    }

    pub fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_expr_deep();
        
        while let Some(_token) = self.peek(){
            let op = match self.peek() {
                Some(Token::Plus) => Ops::Plus,
                Some(Token::Minus) => Ops::Minus,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_expr_deep();
            left = Expr::Operation { a: Box::new(left), op: op, b: Box::new(right) };
        }
        return left;
    }

    pub fn parse_tokens(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt());
        }
        return stmts;
    }
}
