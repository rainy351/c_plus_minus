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
        token
    }

    pub fn expect_ident(&mut self) -> String {
        match self.advance() {
            Some(Token::Ident(name)) => {
                name.clone()
            },
            other => {
                panic!("no ident??? GOT:{:?}", other.unwrap());
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
            other => {
                panic!("no type??? GOT:{:?}", other.unwrap());
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
                    Expr::Call { name, args }
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

                lib_name
            },
            _ => {
                panic!("no libname???");
            }
        }
    }

    pub fn parse_param(&mut self) -> (VarType, String) {
        if self.peek() != Some(&Token::Manual) {
            panic!("parser: expected `manual` as function param. GOT:{:?}", self.peek().unwrap())
        }
        self.advance();
        let var_type = self.expect_type();
        let var_name = self.expect_ident();
        (var_type, var_name)
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
            Some(Token::Ident(var_name)) => {
                let v = var_name.clone();
                let expr = self.parse_expr();
                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                    Stmt::Expr(expr)
                }
                else if self.peek() == Some(&Token::Assign) {
                    self.advance();
                    let new_value = self.parse_expr();
                    if self.peek() != Some(&Token::Semicolon) {
                        panic!("parser: no semicolon???")
                    }
                    else {
                        self.advance();
                    }
                    Stmt::Assignment { var_name: v, new_value: new_value }
                }
                else {
                    panic!("parser: expected a semicolon `;` after expression. GOT:{:?}", self.peek())
                }
            }
            Some(Token::Number(_)) | Some(Token::StringLit(_)) => {
                let expr = self.parse_expr();
                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                    Stmt::Expr(expr)
                }
                else {
                    panic!("parser: expected a semicolon `;` after expression. GOT:{:?}", self.peek())
                }
            },
            Some(Token::Func) => {
                self.advance();
                let name = self.expect_ident();
                if self.peek() != Some(&Token::LParen) {
                    panic!("parser: expected `(` in func declaration")
                }
                let mut args = Vec::new();
                self.advance();
                while self.peek().is_some() && self.peek() != Some(&Token::RParen){
                    args.push(self.parse_param());
                    if self.peek() == Some(&Token::Comma) {
                        self.advance();
                    }
                    else {
                        break;
                    }
                }
                if self.peek() != Some(&Token::RParen) {
                    panic!("parser: expected `)` in func declaration, GOT:{:?}", self.peek().unwrap())
                }
                self.advance();

                if self.peek() != Some(&Token::Colon) {
                    panic!("parser: expected type in func declaration. GOT:{:?}", self.peek())
                }
                self.advance();

                let returns = self.expect_type();

                if self.peek() != Some(&Token::LBrace) {
                    panic!("parser: expected `{}` in func declaration. GOT:{:?}", '{', self.peek())
                }
                self.advance();

                let mut body = Vec::new();

                while self.peek().is_some() && self.peek() != Some(&Token::RBrace) {
                    body.push(self.parse_stmt());
                }

                if self.peek() != Some(&Token::RBrace) {
                    panic!("parser: expected `{}` in func declaration. GOT:{:?}", '}', self.peek())
                }
                self.advance();

                Stmt::FnDecl { fn_name: name, params: args, body, returns }
            },
            Some(Token::Return) => {
                self.advance();
                let value = self.parse_expr();
                if self.peek() != Some(&Token::Semicolon) {
                    panic!("parser: expected semicolon after return. GOT:{:?}", self.peek())
                }
                self.advance();
                Stmt::Return { value }
            },
            Some(Token::If) => {
                self.advance();
                if self.peek() != Some(&Token::LParen) {
                    panic!("parser: expected `(` after `if`");
                }
                self.advance();
                let cond = self.parse_expr();
                if self.peek() != Some(&Token::RParen) {
                    panic!("parser: expected `)` closing condition in `if`");
                }
                self.advance();

                
                if self.peek() != Some(&Token::LBrace) {
                    panic!("parser: expected `{}` after condition in `if`", '{');
                }
                self.advance();

                let mut true_stmts = Vec::new();
                while self.peek().is_some() && self.peek() != Some(&Token::RBrace) {
                    true_stmts.push(self.parse_stmt());
                }
                if self.peek() != Some(&Token::RBrace) {
                    panic!("parser: expected closing `{}` after stmts in `if`", '}')
                }
                self.advance();

                let mut false_stmts_final = Option::None;

                if self.peek() == Some(&Token::Else) {
                    self.advance();
                    if self.peek() != Some(&Token::LBrace) {
                        panic!("parser: expected `{}` after `else`", '{');
                    }
                    self.advance();
                    let mut false_stmts = Vec::new();
                    while self.peek().is_some() && self.peek() != Some(&Token::RBrace) {
                        false_stmts.push(self.parse_stmt());
                    }
                    if self.peek() != Some(&Token::RBrace) {
                        panic!("parser: expected closing `{}` after stmts in `else`", '}')
                    }
                    self.advance();
                    false_stmts_final = Option::Some(false_stmts)
                }

                Stmt::IfElse { condition: cond, true_branch: true_stmts, false_branch: false_stmts_final }
            },
            Some(Token::Else) => {
                panic!("parser: unexpected `else` block. did you forget `if`?")
            },
            Some(Token::While) => {
                self.advance();
                if self.peek() != Some(&Token::LParen) {
                    panic!("parser: expected `(` after `while`");
                }
                self.advance();
                let cond = self.parse_expr();
                if self.peek() != Some(&Token::RParen) {
                    panic!("parser: expected `)` closing condition in `while`");
                }
                self.advance();

                
                if self.peek() != Some(&Token::LBrace) {
                    panic!("parser: expected `{}` after condition in `while`", '{');
                }
                self.advance();

                let mut body = Vec::new();
                while self.peek().is_some() && self.peek() != Some(&Token::RBrace) {
                    body.push(self.parse_stmt());
                }
                if self.peek() != Some(&Token::RBrace) {
                    panic!("parser: expected closing `{}` after stmts in `while`", '}')
                }
                self.advance();

                Stmt::While { condition: cond, body: body }
            },
            _ => panic!("parser: idk what to do ({:?}) :(", self.peek().unwrap())
        }
    }

    pub fn parse_expr_deep(&mut self) -> Expr {
        let mut left = self.parse_primary();
        
        while let Some(_token) = self.peek(){
            let op = match self.peek() {
                Some(Token::Mult) => Ops::Mult,
                Some(Token::Div) => Ops::Div,
                Some(Token::Percent) => Ops::Mod,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_primary();
            left = Expr::Operation { a: Box::new(left), op, b: Box::new(right) };
        }
        left
    }

    pub fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_expr_deep();
        
        while let Some(_token) = self.peek(){
            let op = match self.peek() {
                Some(Token::Plus) => Ops::Plus,
                Some(Token::Minus) => Ops::Minus,
                Some(Token::Gt) => Ops::Gt,
                Some(Token::Lt) => Ops::Lt,
                _ => {
                    break;
                }
            };
            self.advance();
            let right = self.parse_expr_deep();
            left = Expr::Operation { a: Box::new(left), op, b: Box::new(right) };
        }
        left
    }

    pub fn parse_tokens(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }
}
