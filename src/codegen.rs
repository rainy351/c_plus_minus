use core::panic;
use std::{format, str::FromStr};

use crate::ast::{Expr, Ops, Stmt, VarType};

fn op_to_c(op: &Ops) -> String {
    match op {
        Ops::Plus => "+".to_string(),
        Ops::Minus => "-".to_string(),
        Ops::Mult => "*".to_string(),
        Ops::Div => "/".to_string(),
        _ => panic!("codegen: cant translate op to c: {:?}", op),
    }
}

fn var_type_to_c(var_type: &VarType) -> String{
    match var_type {
        VarType::TypeInt => "long long".to_string(),
        VarType::TypeString => "const char*".to_string(),
        VarType::Idk => "long long".to_string() // "auto".to_string(),
    }
}

pub fn expr_to_c(expr: &Expr) -> String {
    match expr {
        Expr::Number(num) => num.to_string(),
        Expr::StringLit(stringlit) => format!("\"{}\"", stringlit),
        Expr::VarIdent(var_name) => var_name.to_string(),
        Expr::Operation { a, op, b } => format!("({} {} {})", expr_to_c(a), op_to_c(op), expr_to_c(b)),
        Expr::Call { name, args } => {
            // let call_args: Vec<String> = args.iter().map(|l_expr| expr_to_c(l_expr)).collect();
            // format!("{}({})", name, call_args.join(", "))
            let mut cool_str = format!("{}(", name);
            for (i, expr) in args.iter().enumerate() {
                if i > 0 {
                    cool_str += ", ";
                }
                cool_str += &expr_to_c(expr);
            };
            cool_str += ")";
            cool_str
        },
        _ => panic!("codegen: expr not implemented!!!")
    }
}

pub fn stmt_to_c(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Include { lib_name } => format!("/*include <{lib_name}>*/\n"),
        Stmt::Variable { var_name, var_type, var_value } => format!("{} {} = {};\n", var_type_to_c(var_type), var_name, expr_to_c(var_value)),
        Stmt::Assignment { var_name, new_value } => format!("{} = {};\n", var_name, expr_to_c(new_value)),
        Stmt::PressF { var_name } => format!("/* F {}; <- this is just a placholder for now */\n", var_name),
        Stmt::Expr(expr) => format!("{};\n", expr_to_c(expr)),
        _ => panic!("codegen: stmt not implemented!!! GOT:{:?}", stmt)
    }
}

pub fn generate_c_code(stmts: &[Stmt]) -> String {
    let mut c_code = String::from("#include <stdio.h>\n#include <stdbool.h>\n\nint main() {\n");

    for stmt in stmts {
        c_code.push_str(&stmt_to_c(stmt));
    }

    c_code.push_str("    return 0;\n}\n");
    c_code
}
