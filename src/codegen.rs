use crate::ast::{Ops, VarType};
use crate::typechecker::{TypedExpr, TypedStmt};

fn op_to_c(op: &Ops) -> String {
    match op {
        Ops::Plus => "+".to_string(),
        Ops::Minus => "-".to_string(),
        Ops::Mult => "*".to_string(),
        Ops::Div => "/".to_string(),
        Ops::Lt => "<".to_string(),
        Ops::Gt => ">".to_string(),
        _ => panic!("codegen: cant translate op to c: {:?}", op),
    }
}

fn var_type_to_c(var_type: &VarType) -> String{
    match var_type {
        VarType::TypeInt => "long long".to_string(),
        VarType::TypeString => "const char*".to_string(),
        VarType::Idk => unreachable!() // "long long".to_string() // "auto".to_string(),
    }
}

fn parse_fparams(params: &Vec<(VarType, String)>) -> String {
    let mut params_c = Vec::new();
    for (var_type, var_name) in params {
        params_c.push(format!("{} {}", var_type_to_c(&var_type), var_name));
    }
    params_c.join(", ")
}
    
pub fn mangle_func_name(name: &String, args: &Vec<(VarType, String)>) -> String {
    let mut suffix = String::new();
    for (var_type, _) in args {
        suffix.push(match var_type {
            &VarType::TypeInt => 'i',
            &VarType::TypeString => 's',
            _ => panic!("codegen: var type not implemented(at func args)!!!")
        });
    };
    return format!("{}__{}", name, suffix)
}

pub fn expr_to_c(expr: &TypedExpr) -> String {
    match expr {
        TypedExpr::Number(num) => num.to_string(),
        TypedExpr::StringLit(stringlit) => format!("\"{}\"", stringlit),
        TypedExpr::VarIdent { name: var_name, var_type } => var_name.to_string(),
        TypedExpr::Operation { a, op, b, return_type } => format!("({} {} {})", expr_to_c(a), op_to_c(op), expr_to_c(b)),
        TypedExpr::Call { name, args, return_type } => {
            let mut args_typed = Vec::new();
            for expr in args {
                match expr {                            
                    TypedExpr::StringLit(string_lit) => {
                        args_typed.push((VarType::TypeString, "".to_string()));
                    },
                    TypedExpr::Number(num) => {
                        args_typed.push((VarType::TypeInt, "".to_string()));
                    },
                    TypedExpr::VarIdent { name: var_name, var_type } => {
                        args_typed.push((var_type.clone(), "".to_string()));
                    },
                    TypedExpr::Call { name, args, return_type } => {
                        args_typed.push((return_type.clone(), "".to_string()));
                    },
                    TypedExpr::Operation { a, op, b, return_type } => {
                        args_typed.push((return_type.clone(), "".to_string()));
                    }
                    other => {
                        panic!("codegen: expr not implemented! GOT:{:?}", other)
                    }
                }
            }
            match name.as_str() {
                "__builtin_sys_print" => {
                    if args.len() > 1 { panic!("codegen: sorry pal, `__builtin_sys_print` only supports 1 arg for now") }
                    let arg = args.get(0).expect("codegen: at least 1 arg expected at: __builtin_sys_print");
                    let (var_type, var_name) = args_typed.get(0).expect("codegen: at least 1 arg expected at: `__builtin_sys_print`");
                    match var_type {
                        VarType::TypeInt => {
                            return format!("printf(\"%lld\", {})", expr_to_c(arg))
                        },
                        VarType::TypeString => {
                            return format!("printf(\"%s\", {})", expr_to_c(arg))
                        },
                        VarType::Idk => unreachable!()
                    }
                }
                _ => {    
                    let mut cool_str = format!("{}(", mangle_func_name(name, &args_typed));
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

pub fn stmt_to_c(stmt: &TypedStmt) -> String {
    match stmt {
        TypedStmt::Variable { var_name, var_type, var_value } => {
            return format!("{} {} = {};\n", var_type_to_c(var_type), var_name, expr_to_c(var_value))
        },
        TypedStmt::Assignment { var_name, new_value, var_type } => format!("{} = {};\n", var_name, expr_to_c(new_value)),
        TypedStmt::PressF { var_name } => format!("/* F {}; <- this is just a placeholder for now */\n", var_name),
        TypedStmt::Expr(expr) => format!("{};\n", expr_to_c(expr)),
        TypedStmt::FnDecl { fn_name, params, body , returns} => {
            let mut body_as_str = String::new();
            for stmt in body {
                body_as_str.push_str(&stmt_to_c(stmt));
            }
            format!("{} {}({})", var_type_to_c(returns), mangle_func_name(fn_name, params), parse_fparams(params)) + "{\n" + &body_as_str + "return 0;\n}"
        },
        TypedStmt::Return { value } => format!("return {};\n", expr_to_c(value)),
        TypedStmt::IfElse { condition, true_branch, false_branch } => {
            let mut ifelse_final = format!("if ({}) {}\n", expr_to_c(condition), '{');
            for stmt in true_branch {
                ifelse_final.push_str(&stmt_to_c(stmt));
            }
            ifelse_final.push_str("}\n");
            if let Some(else_stmts) = false_branch {
                ifelse_final.push_str("else {\n");
                for stmt in else_stmts {
                    ifelse_final.push_str(&stmt_to_c(&stmt));
                }
                ifelse_final.push_str("}\n");
            }
            ifelse_final
        },
        _ => panic!("codegen: stmt not implemented!!! GOT:{:?}", stmt)
    }
}

pub fn generate_c_code(stmts: &Vec<TypedStmt>) -> String {
    let mut c_code = String::from("// CODE GENERATED WITH C± TO C TRANSLATOR!!! NOT FOR MANUAL EDITING!\n#include <stdio.h>\n#include <stdbool.h>\n\n");

    c_code.push_str("// FUNC PROTOTYPES!!!\n");
    for stmt in stmts {
        if let TypedStmt::FnDecl { fn_name, params, body, returns } = stmt {
            // c_code.push_str(&stmt_to_c(stmt));
            if params.len() == 0 {
                c_code.push_str(&format!("{} {}(void);\n", var_type_to_c(returns), mangle_func_name(fn_name, params)));
            }
            else {
                c_code.push_str(&format!("{} {}({});\n", var_type_to_c(returns), mangle_func_name(fn_name, params), parse_fparams(params)));
            }
        }
    }

    c_code.push_str("\n// FUNCS!!!\n");
    
    for stmt in stmts {
        if let TypedStmt::FnDecl { .. } = stmt {
            c_code.push_str(&stmt_to_c(stmt));
            c_code.push_str("\n\n");
        }
    }

    c_code.push_str("\n// CODE!!!\nint main() {\n");

    for stmt in stmts {
        if let TypedStmt::FnDecl { .. } = stmt {}
        else {
            c_code.push_str(&stmt_to_c(stmt));
        }
    }

    c_code.push_str("    return 0;\n}\n");
    c_code
}
