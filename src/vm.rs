use core::panic;
use std::{collections::HashMap, println};
use crate::{ast::{Expr, Stmt, VarType, Ops}};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
    Null,
}

impl Value {
    pub fn as_str(&self) -> String {
        match self {
            Value::String(val) => {
                return val.clone();
            },
            _ => panic!("is not a string!")
        }
    }

    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(val) => {
                return *val;
            },
            _ => panic!("is not an int")
        }
    }
}

pub struct VM {
    memory: HashMap<String, (VarType, Value)>,
}

impl VM {
    pub fn new() -> Self {
        return Self {
            memory: HashMap::new(),
        }
    }

    fn check_type(&self, var_type:&VarType, value:&Value) -> bool {
        match (var_type, value) {
            (VarType::TypeInt, Value::Int(_)) => true,
            (VarType::TypeString, Value::String(_)) => true,
            (VarType::Idk, _) => true,
            _ => false
        }
    }

    pub fn eval_expr(&self, expr: &Expr) -> Value {
        match &expr {
            Expr::Number(num) => {
                return Value::Int(*num);
            },
            Expr::StringLit(stringlit) => {
                return Value::String(stringlit.clone());
            },
            Expr::VarIdent(varname) => {
                return self.memory.get(varname).unwrap().1.clone();
            },
            Expr::Operation{ a, op, b } => {
                let left  = self.eval_expr(a).as_int();
                let right = self.eval_expr(b).as_int();
                let res;
                match op {
                    Ops::Plus => {
                        res = left + right;
                    },
                    Ops::Minus => {
                        res = left - right;
                    },
                    Ops::Mult => {
                        res = left * right;
                    },
                    Ops::Div => {
                        res = left / right;
                    },
                    _ => { panic!("op not implemented!!!") }
                }
                return Value::Int(res);
            }
            _ => { panic!("expr not implemented!!!") }
        }

        Value::Null
    }

    pub fn execute(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match stmt {
                Stmt::Variable { var_name, var_type, var_value } => {
                    let val = self.eval_expr(&var_value);
                    if self.check_type(&var_type, &val) {
                        println!("Inserted var: name={}, type={:?}, val={:?}", var_name, var_type, val);
                        self.memory.insert(var_name, (var_type, val));
                    }
                    else {
                        panic!("tried to assign wrong types: VAR_NAME={:?}, TYPE={:?}, VALUE={:?}", var_name, var_type, val)
                    }
                    
                },
                Stmt::PressF { var_name } => {
                    println!("Removed var: name={}", var_name);
                    self.memory.remove(&var_name);
                },
                _ => {}
            }
        }
        println!("FINISHED. MEMORY: {:?}", self.memory);
    }
}
