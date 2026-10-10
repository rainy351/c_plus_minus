use std::collections::HashMap;

use crate::ast::{Expr, Ops, Stmt, VarType};

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
        VarType::Idk => "long long".to_string() // "auto".to_string(),
    }
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

pub fn expr_to_c(expr: &Expr, scope: &Vec<HashMap<String, VarType>>) -> String {
    match expr {
        Expr::Number(num) => num.to_string(),
        Expr::StringLit(stringlit) => format!("\"{}\"", stringlit),
        Expr::VarIdent(var_name) => var_name.to_string(),
        Expr::Operation { a, op, b } => format!("({} {} {})", expr_to_c(a, scope), op_to_c(op), expr_to_c(b, scope)),
        Expr::Call { name, args } => {
            match name.as_str() {
                "__builtin_sys_print" => {
                    if args.len() > 1 { panic!("codegen: sorry pal, `__builtin_sys_print` only supports 1 arg for now") }
                    let arg = args.get(0).expect("codegen: at least 1 arg expected at: `__builtin_sys_print`");
                    match arg {
                        Expr::StringLit(_) => {
                            return format!("printf(\"%s\", {})", expr_to_c(arg, scope))
                        },
                        Expr::Number(_) => {
                            return format!("printf(\"%lld\", {})", expr_to_c(arg, scope))
                        },
                        Expr::VarIdent(var_name) => {
                            for next_scope in scope.iter().rev() {
                                if next_scope.get(var_name).is_some() {
                                    match next_scope.get(var_name).unwrap() {
                                        VarType::TypeInt => return format!("printf(\"%lld\", {})", expr_to_c(arg, scope)),
                                        VarType::TypeString => return format!("printf(\"%s\", {})", expr_to_c(arg, scope)),
                                        _ => panic!("codegen: cartype not implemented in func call")
                                    }
                                }
                            };
                            return panic!("codegen: var {} not found in this scope.", var_name);
                        },
                        Expr::Call { name, args } => {
                            return format!("printf(\"%lld\", {})", expr_to_c(arg, scope))
                        },
                        other => {
                            return panic!("codegen: expr not implemented! GOT:{:?}", other)
                        }
                    }
                },
                other => {
                    // let call_args: Vec<String> = args.iter().map(|l_expr| expr_to_c(l_expr)).collect();
                    // format!("{}({})", name, call_args.join(", "))
                    
                    let mut args_typed = Vec::new();
                    for expr in args {
                        match expr {                            
                            Expr::StringLit(string_lit) => {
                                args_typed.push((VarType::TypeString, "".to_string()));
                            },
                            Expr::Number(num) => {
                                args_typed.push((VarType::TypeInt, "".to_string()));
                            },
                            Expr::VarIdent(var_name) => {
                                let mut found = false;
                                for next_scope in scope.iter().rev() {
                                    if next_scope.get(var_name).is_some() {
                                        match next_scope.get(var_name).unwrap() {
                                            VarType::TypeInt => {args_typed.push((VarType::TypeInt, "".to_string())); found=true; break;},
                                            VarType::TypeString => {args_typed.push((VarType::TypeString, "".to_string())); found=true; break;},
                                            _ => panic!("codegen: cartype not implemented in expr_to_c")
                                        }
                                    }
                                }
                                if !found {
                                    panic!("codegen: var {} not found in this scope.", var_name)
                                }
                            },
                            // Expr::Call { name, args } => {
                            //     return format!("printf(\"%lld\", {})", expr_to_c(arg, scope))
                            // },
                            other => {
                                panic!("codegen: expr not implemented! GOT:{:?}", other)
                            }
                        }
                        
                        // args_typed.push(format!("{} {}", var_type_to_c(&var_type), var_name));
                    }

                    let mut cool_str = format!("{}(", mangle_func_name(name, &args_typed));
                    for (i, expr) in args.iter().enumerate() {
                        if i > 0 {
                            cool_str += ", ";
                        }
                        cool_str += &expr_to_c(expr, scope);
                    };
                    cool_str += ")";
                    cool_str
                }
            }
        },
        _ => panic!("codegen: expr not implemented!!!")
    }
}

pub fn stmt_to_c(stmt: &Stmt, scope: &mut Vec<HashMap<String, VarType>>) -> String {
    match stmt {
        Stmt::Include { lib_name } => "".to_string(), //format!("/*include <{lib_name}>*/\n"),
        Stmt::Variable { var_name, var_type, var_value } => {
            scope.last_mut().unwrap().insert(var_name.to_string(), var_type.clone());
            return format!("{} {} = {};\n", var_type_to_c(var_type), var_name, expr_to_c(var_value, scope))
        },
        Stmt::Assignment { var_name, new_value } => format!("{} = {};\n", var_name, expr_to_c(new_value, scope)),
        Stmt::PressF { var_name } => format!("/* F {}; <- this is just a placholder for now */\n", var_name),
        Stmt::Expr(expr) => format!("{};\n", expr_to_c(expr, scope)),
        Stmt::FnDecl { fn_name, params, body , returns} => {
            scope.push(HashMap::new());
            let mut body_as_str = String::new();
            let mut params_c = Vec::new();
            for (var_type, var_name) in params {
                scope.last_mut().unwrap().insert(var_name.to_string(), var_type.clone());
                params_c.push(format!("{} {}", var_type_to_c(&var_type), var_name));
            }
            for stmt in body {
                body_as_str.push_str(&stmt_to_c(stmt, scope));
            }
            scope.pop();
            format!("{} {}({})", var_type_to_c(returns), mangle_func_name(fn_name, params), params_c.join(", ")) + "{\n" + &body_as_str + "return 0;\n}"
        },
        Stmt::Return { value } => format!("return {};\n", expr_to_c(value, scope)),
        Stmt::IfElse { condition, true_branch, false_branch } => {
            scope.push(HashMap::new());
            let mut ifelse_final = format!("if ({}) {}\n", expr_to_c(condition, scope), '{');
            for stmt in true_branch {
                ifelse_final.push_str(&stmt_to_c(stmt, scope));
            }
            ifelse_final.push_str("}\n");
            if let Some(else_stmts) = false_branch {
                ifelse_final.push_str("else {\n");
                for stmt in else_stmts {
                    ifelse_final.push_str(&stmt_to_c(&stmt, scope));
                }
                ifelse_final.push_str("}\n");
            }
            scope.pop();

            ifelse_final
        },
        _ => panic!("codegen: stmt not implemented!!! GOT:{:?}", stmt)
    }
}

pub fn generate_c_code(stmts: &[Stmt]) -> String {
    let mut c_code = String::from("// CODE GENERATED WITH C± TO C TRANSLATOR!!! NOT FOR MANUAL EDITING!\n#include <stdio.h>\n#include <stdbool.h>\n\n");
    let mut scope: Vec<HashMap<String, VarType>> = Vec::new();

    scope.push(HashMap::new());

    for stmt in stmts {
        if let Stmt::FnDecl { .. } = stmt {
            c_code.push_str(&stmt_to_c(stmt, &mut scope));
            c_code.push_str("\n\n");
        }
    }

    c_code.push_str("\n\nint main() {\n");

    for stmt in stmts {
        if let Stmt::FnDecl { .. } = stmt {}
        else {
            c_code.push_str(&stmt_to_c(stmt, &mut scope));
        }
    }

    c_code.push_str("    return 0;\n}\n");
    c_code
}
