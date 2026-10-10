use std::{collections::HashMap, mem::take};
use crate::ast::{Expr, Ops, Stmt, VarType};


struct FuncSignature {
    name: String,
    args: Vec<VarType>,
    returns: VarType,
}

impl FuncSignature {
    fn new(name: &String, args: &Vec<(VarType, String)>, returns: &VarType) -> FuncSignature {
        let mut args_new = Vec::new();
        for (avtype, _) in args {
            args_new.push(avtype.clone());
        }
        FuncSignature{name: name.clone(), args: args_new, returns: returns.clone()}
    }
}

pub struct TypeChecker {
    ast: Vec<Stmt>,
    scopes: Vec<HashMap<String, VarType>>,
    fsigns: HashMap<(String, Vec<VarType>), FuncSignature>,
}

impl TypeChecker {
    pub fn new(mut ast: Vec<Stmt>) -> TypeChecker{
        let mut scopes: Vec<HashMap<String, VarType>> = Vec::new();
        scopes.push(HashMap::new());

        let fsigns = TypeChecker::get_fsigns(&ast);

        TypeChecker { ast: ast, scopes: scopes, fsigns: fsigns }
    }

    fn get_fsigns(ast: &Vec<Stmt>) -> HashMap<(String, Vec<VarType>), FuncSignature> {
        let mut fsigns = HashMap::new();
        for stmt in ast {
            match stmt {
                Stmt::FnDecl { fn_name, params, body, returns } => {
                    let mut args = Vec::new();
                    for (vt, s) in params {
                        args.push(vt.clone());
                    }
                    fsigns.insert((fn_name.to_string(), args), FuncSignature::new(fn_name, params, returns));
                }
                _ => {}
            }
        }
        return fsigns;
    }

    fn exprs_to_vartypes(&self, exprs: Vec<Expr>) -> Vec<VarType> {
        let mut exprs_parsed = Vec::new();
        for a in exprs {
            exprs_parsed.push(self.expr_to_vartype(a.clone()));
        }
        exprs_parsed
    }

    fn expr_to_vartype(&self, e: Expr) -> VarType{
        match e {
            Expr::Number(_) => VarType::TypeInt,
            Expr::StringLit(_) => VarType::TypeString,
            Expr::VarIdent(name) => self.get_var_type(&name),
            Expr::Operation { a, op, b } => {
                let a_type = self.expr_to_vartype(*a);
                let b_type = self.expr_to_vartype(*b);
                if a_type == b_type {
                    match &a_type {
                        VarType::TypeString => {
                            match &op {
                                Ops::Plus => panic!("typecheker: not implemented op for type {:?}: {:?}", a_type, op),
                                other => panic!("typecheker: unsupported op for type {:?}: {:?}", a_type, other),
                            }
                        },
                        VarType::Idk => {
                            return panic!("typecheker: unable to define operands' types")
                        },
                        _ => a_type,
                    }
                }
                else {
                    return panic!("typecheker: unexpected operation?")
                }
            },
            Expr::Call { name, args } => {
                let k = (name, self.exprs_to_vartypes(args));
                let a = self.fsigns.get(&k);
                if a.is_some() {
                    return a.unwrap().returns.clone();
                }
                else {
                    panic!("typechecker: unknown function called:{:?}", k.0)
                }
            }
            _ => panic!("typechecker: cant convert expr to VarType:{:?}", e)
        }
    }

    fn get_var_type(&self, name: &String) -> VarType{
        for scope in self.scopes.iter().rev() {
            if scope.get(name).is_some() {
                return scope.get(name).unwrap().clone();
            }
        }
        return panic!("codegen: var {:?} not found in this scope. SCOPE:{:?}", name, self.scopes);
    }

    pub fn typecheck(&mut self) -> &Vec<Stmt> {
        let mut ast = take(&mut self.ast);
        for node in ast.iter_mut() {
            match node {
                Stmt::Variable { var_name, var_type, var_value } => {
                    match *var_type {
                        VarType::Idk => {
                            match var_value {
                                Expr::StringLit(string_lit) => {
                                    let v = self.expr_to_vartype(var_value.clone());
                                    self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
                                    *var_type = VarType::TypeString;
                                },
                                Expr::Number(num) => {
                                    let v = self.expr_to_vartype(var_value.clone());
                                    self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
                                    *var_type = VarType::TypeInt;
                                },
                                Expr::Call { name, args } => {

                                    let b = (name.clone(), self.exprs_to_vartypes(args.clone()));
                                    if self.fsigns.get(&b).is_some() {
                                        *var_type = self.fsigns.get(&b).unwrap().returns.clone();
                                        self.scopes.last_mut().unwrap().insert(var_name.to_string(), var_type.clone());
                                    }
                                    else {
                                        panic!("typechecker: unidentified function call:{:?}", name)
                                    }
                                },
                                Expr::VarIdent(name) => {
                                    let v = self.get_var_type(name);
                                    *var_type = v.clone();
                                    self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
                                },
                                Expr::Operation { a, op, b } => {
                                    let v = self.expr_to_vartype(var_value.clone());
                                    *var_type = v.clone();
                                    self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
                                },
                                _ => {/* idc */}
                            }
                        },
                        _ => {
                            self.scopes.last_mut().unwrap().insert(var_name.to_string(), var_type.clone());
                        }
                    }
                }
                _ => {/*idc*/}
            }
        }
        self.ast = ast;
        &self.ast
    }
        
}
