use core::panic;

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
            match name.as_str() {
                "__builtin_sys_print" => {
                    if args.len() > 1 { panic!("codegen: sorry pal, `__builtin_sys_print` only supports 1 arg for now") }
                    let arg = args.get(0).expect("codegen: at least 1 arg expected at: `__builtin_sys_print`");
                    if let Expr::StringLit(string_lit) = arg {
                        return format!("printf(\"%s\", {})", expr_to_c(arg))
                    }
                    else {
                        return format!("printf(\"%lld\", {})", expr_to_c(arg))
                    }
                },
                other => {
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
                }
            }
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
        Stmt::FnDecl { fn_name, params, body } => {
            let mut body_as_str = String::new();
            for stmt in body {
                body_as_str.push_str(&stmt_to_c(stmt));
            }
            let params_c: Vec<String> = params.iter().map(|p| format!("long long {}", p)).collect();
            format!("long long {}({})", fn_name, params_c.join(", ")) + "{\n" + &body_as_str + "return 0;\n}"
        },
        Stmt::Return { value } => format!("return {};\n", expr_to_c(value)),
        _ => panic!("codegen: stmt not implemented!!! GOT:{:?}", stmt)
    }
}

pub fn generate_c_code(stmts: &[Stmt]) -> String {
    let mut c_code = String::from("// CODE GENERATED WITH C± TO C TRANSLATOR!!! NOT FOR MANUAL EDITING!\n#include <stdio.h>\n#include <stdbool.h>\n\n");

    for stmt in stmts {
        if let Stmt::FnDecl { .. } = stmt {
            c_code.push_str(&stmt_to_c(stmt));
            c_code.push_str("\n\n");
        }
    }

    c_code.push_str("\n\nint main() {\n");

    for stmt in stmts {
        if let Stmt::FnDecl { .. } = stmt {}
        else {
            c_code.push_str(&stmt_to_c(stmt));
        }
    }

    c_code.push_str("    return 0;\n}\n");
    c_code
}
