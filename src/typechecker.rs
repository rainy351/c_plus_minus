use std::{collections::HashMap, mem::take, panic, unreachable};
use crate::ast::{Expr, Ops, Stmt, VarType};


#[derive(Debug, Clone, PartialEq)]
pub enum TypedExpr {
    Number(i64),
    StringLit(String),
    VarIdent{
        name: String,
        var_type: VarType,
    },
    Operation {
        a: Box<TypedExpr>,
        op: Ops,
        b: Box<TypedExpr>,
        return_type: VarType,
    },
    Call {
        name: String,
        args: Vec<TypedExpr>,
        return_type: VarType,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedStmt {
    Variable {
        var_name:  String,
        var_type:  VarType,
        var_value: TypedExpr,
    },
    Assignment {
        var_name:  String,
        new_value: TypedExpr,
        var_type: VarType,
    },
    PressF {
        var_name:  String,
    },
    FnDecl {
        fn_name: String,
        params: Vec<(VarType, String)>,
        body: Vec<TypedStmt>,
        returns: VarType,
    },
    Return {
        value: TypedExpr,
    },
    IfElse {
        condition: TypedExpr,
        true_branch: Vec<TypedStmt>,
        false_branch: Option<Vec<TypedStmt>>,
    },
    Expr(TypedExpr),
}


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
    ast: Vec<TypedStmt>,
    scopes: Vec<HashMap<String, VarType>>,
    fsigns: HashMap<(String, Vec<VarType>), FuncSignature>,
}

impl TypeChecker {
    pub fn new(mut ast: Vec<Stmt>) -> TypeChecker{
        let mut scopes: Vec<HashMap<String, VarType>> = Vec::new();
        scopes.push(HashMap::new());

        let fsigns = TypeChecker::get_fsigns(&ast);

        let mut checker = TypeChecker { 
            ast: Vec::<TypedStmt>::new(),
            scopes: scopes,
            fsigns: fsigns 
        };
        checker.convert_ast(ast);
        checker
    }

    fn texpr_to_vartype(&self, e: &TypedExpr) -> VarType{
        match e {
            TypedExpr::Number(_) => VarType::TypeInt,
            TypedExpr::StringLit(_) => VarType::TypeString,
            TypedExpr::VarIdent { name, var_type } => var_type.clone(),
            TypedExpr::Operation { a, op, b , return_type} => { return_type.clone() },
            TypedExpr::Call { name, args , return_type} => return_type.clone(),
            _ => panic!("typechecker: cant convert expr to VarType:{:?}", e)
        }
    }

    fn texprs_to_vartypes(&self, exprs: Vec<TypedExpr>) -> Vec<VarType> {
        let mut exprs_parsed = Vec::new();
        for a in exprs {
            exprs_parsed.push(self.texpr_to_vartype(&a));
        }
        exprs_parsed
    }

    fn convert_expr(&mut self, expr: Expr) -> TypedExpr {
        match expr {
            Expr::Number(num) => TypedExpr::Number(num),
            Expr::StringLit(string_lit) => TypedExpr::StringLit(string_lit),
            Expr::Operation { a, op, b } => {
                let typed_a = self.convert_expr(*a);
                let typed_b = self.convert_expr(*b);
                let a_t = self.texpr_to_vartype(&typed_a);
                let b_t = self.texpr_to_vartype(&typed_b);
                if a_t != b_t {
                    panic!("typechecker: sorry pal, you cant use different types of operands yet. GOT:({:?};{:?})", a_t, b_t)
                }
                else {
                    match a_t {
                        VarType::TypeInt => {
                            TypedExpr::Operation { a: Box::new(typed_a), op, b: Box::new(typed_b), return_type: VarType::TypeInt }
                        },
                        VarType::TypeString => {
                            match op {
                                Ops::Plus => TypedExpr::Operation { a: Box::new(typed_a), op, b: Box::new(typed_b), return_type: VarType::TypeString },
                                Ops::Mult => TypedExpr::Operation { a: Box::new(typed_a), op, b: Box::new(typed_b), return_type: VarType::TypeString },
                                _ => {
                                    panic!("typechecker: op {:?} is not supported for type {:?}", op, a_t)
                                }
                            }
                        }
                        VarType::Idk => {
                            panic!("typechecker: how did idk type get into typedast op lol?")
                        }
                    }
                    
                }
            },
            Expr::VarIdent(name) => {
                let t = self.get_var_type(&name);
                TypedExpr::VarIdent { name, var_type: t }
            },
            Expr::Call { name, args } => {
                let mut targs = Vec::new();
                let mut targ_types = Vec::new();
                for arg in args {
                    let targ = self.convert_expr(arg);
                    targ_types.push(self.texpr_to_vartype(&targ));
                    targs.push(targ);
                }
                let s = (name, targ_types);
                let fsign = self.fsigns.get(&s);
                if fsign.is_none() {
                    panic!("typechecker: func {:?} not found!", s.0)
                }
                let return_type = fsign.expect("typechecker: cannot unpack return_type").returns.clone();
                TypedExpr::Call { name: s.0, args: targs, return_type: return_type }
            },
        }
    }

    fn check_scope(&self, var_name: &str) {
        if self.scopes.last().unwrap().get(var_name).is_some() {
            panic!("typechecker: var {:?} is already declared in this scope", var_name)
        }
    }

    fn convert_stmt(&mut self, stmt: Stmt, expected_return: Option<&VarType>) -> TypedStmt {
        match stmt {
            Stmt::Include { lib_name } => { unreachable!() },
            Stmt::Variable { var_name, var_type, var_value } => {
                let t = self.convert_expr(var_value);
                self.check_scope(&var_name);
                let val_type = self.texpr_to_vartype(&t);
                if var_type != VarType::Idk {
                    if val_type != var_type {
                       panic!("typechecker: tried to assign value of type {:?} to a {:?} type var", val_type, var_type)
                    }
                }
                self.scopes.last_mut().unwrap().insert(var_name.clone(), val_type.clone());
                TypedStmt::Variable { var_name, var_type: val_type, var_value: t}
            },
            Stmt::Assignment { var_name, new_value } => {
                let t_value = self.convert_expr(new_value);
                let var_type = self.get_var_type(&var_name);
                if var_type != self.texpr_to_vartype(&t_value) {
                    panic!("typechecker: tried to assign value of type {:?} to a {:?} type var", self.texpr_to_vartype(&t_value), var_type)
                }
                TypedStmt::Assignment { var_name: var_name, new_value: t_value, var_type: var_type}
            },
            Stmt::Expr(expr) => {
                TypedStmt::Expr(self.convert_expr(expr))
            },
            Stmt::Return { value } => {
                let t_value = self.convert_expr(value);
                let value_type = self.texpr_to_vartype(&t_value);
                if &value_type != expected_return.expect("typechecker: return outside of function = bad. dont do zis") {
                    panic!("typechecker: tried to return incompatible type: func type {:?}, val type {:?}", expected_return, value_type)
                }
                TypedStmt::Return { value: t_value }
            },
            Stmt::FnDecl { fn_name, params, body, returns } => {
                self.scopes.push(HashMap::new());
                for (param_type, param_name) in &params {
                    self.check_scope(param_name);
                    self.scopes.last_mut().unwrap().insert(param_name.clone(), param_type.clone());
                }
                let mut t_stmts = Vec::new();
                for stmt in body {
                    t_stmts.push(self.convert_stmt(stmt, Some(&returns)));
                }
                self.scopes.pop();
                TypedStmt::FnDecl { fn_name, params, body: t_stmts, returns }
            },
            Stmt::IfElse { condition, true_branch, false_branch } => {
                let t_condition = self.convert_expr(condition);
                self.scopes.push(HashMap::new());
                let mut t_true_branch = Vec::new();
                for stmt in true_branch {
                    t_true_branch.push(self.convert_stmt(stmt, expected_return));
                }
                self.scopes.pop();
                let t_false_branch = match false_branch {
                    Some(stmts) => {
                        self.scopes.push(HashMap::new());
                        let mut tfb_vec = Vec::new();
                        for stmt in stmts {
                            tfb_vec.push(self.convert_stmt(stmt, expected_return));
                        }
                        self.scopes.pop();
                        Some(tfb_vec)
                    },
                    None => { None }
                };
                TypedStmt::IfElse { condition: t_condition, true_branch: t_true_branch, false_branch: t_false_branch }
            },
            Stmt::PressF { var_name } => {
                TypedStmt::PressF { var_name: var_name }
            },
        }
    }

    fn convert_ast(&mut self, mut ast: Vec<Stmt>) {
        for stmt in ast {
            let typed_stmt = self.convert_stmt(stmt, None);
            self.ast.push(typed_stmt);
        }
    }

    fn get_fsigns(ast: &Vec<Stmt>) -> HashMap<(String, Vec<VarType>), FuncSignature> {
        let mut fsigns = HashMap::new();
        let builtin_print = "__builtin_sys_print".to_string();
        fsigns.insert((builtin_print.clone(), vec![VarType::TypeInt]), FuncSignature::new(&builtin_print, &vec![(VarType::TypeInt, "int".to_string())], &VarType::TypeInt));
        fsigns.insert((builtin_print.clone(), vec![VarType::TypeString]), FuncSignature::new(&builtin_print, &vec![(VarType::TypeString, "text".to_string())], &VarType::TypeString));
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

    fn get_var_type(&self, name: &String) -> VarType{
        for scope in self.scopes.iter().rev() {
            if scope.get(name).is_some() {
                return scope.get(name).unwrap().clone();
            }
        }
        return panic!("codegen: var {:?} not found in this scope. SCOPE:{:?}", name, self.scopes);
    }

    // fn exprs_to_vartypes(&self, exprs: Vec<Expr>) -> Vec<VarType> {
    //     let mut exprs_parsed = Vec::new();
    //     for a in exprs {
    //         exprs_parsed.push(self.expr_to_vartype(a.clone()));
    //     }
    //     exprs_parsed
    // }

    // fn expr_to_vartype(&self, e: Expr) -> VarType{
    //     match e {
    //         Expr::Number(_) => VarType::TypeInt,
    //         Expr::StringLit(_) => VarType::TypeString,
    //         Expr::VarIdent(name) => self.get_var_type(&name),
    //         Expr::Operation { a, op, b } => {
    //             let a_type = self.expr_to_vartype(*a);
    //             let b_type = self.expr_to_vartype(*b);
    //             if a_type == b_type {
    //                 match &a_type {
    //                     VarType::TypeString => {
    //                         match &op {
    //                             Ops::Plus => panic!("typecheker: not implemented op for type {:?}: {:?}", a_type, op),
    //                             other => panic!("typecheker: unsupported op for type {:?}: {:?}", a_type, other),
    //                         }
    //                     },
    //                     VarType::Idk => {
    //                         return panic!("typecheker: unable to define operands' types")
    //                     },
    //                     _ => a_type,
    //                 }
    //             }
    //             else {
    //                 return panic!("typecheker: unexpected operation?")
    //             }
    //         },
    //         Expr::Call { name, args } => {
    //             let k = (name, self.exprs_to_vartypes(args));
    //             let a = self.fsigns.get(&k);
    //             if a.is_some() {
    //                 return a.unwrap().returns.clone();
    //             }
    //             else {
    //                 return panic!("typechecker: unknown function called:{:?}", k.0)
    //             }
    //         }
    //         _ => panic!("typechecker: cant convert expr to VarType:{:?}", e)
    //     }
    // }

    // pub fn typecheck(&mut self) -> &Vec<Stmt> {
    //     let mut ast = take(&mut self.ast);
    //     for node in ast.iter_mut() {
    //         match node {
    //             Stmt::Variable { var_name, var_type, var_value } => {
    //                 match *var_type {
    //                     VarType::Idk => {
    //                         match var_value {
    //                             Expr::StringLit(string_lit) => {
    //                                 let v = self.expr_to_vartype(var_value.clone());
    //                                 self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
    //                                 *var_type = VarType::TypeString;
    //                             },
    //                             Expr::Number(num) => {
    //                                 let v = self.expr_to_vartype(var_value.clone());
    //                                 self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
    //                                 *var_type = VarType::TypeInt;
    //                             },
    //                             Expr::Call { name, args } => {
    //                                 let b = (name.clone(), self.exprs_to_vartypes(args.clone()));
    //                                 if self.fsigns.get(&b).is_some() {
    //                                     *var_type = self.fsigns.get(&b).unwrap().returns.clone();
    //                                     self.scopes.last_mut().unwrap().insert(var_name.to_string(), var_type.clone());
    //                                 }
    //                                 else {
    //                                     panic!("typechecker: unidentified function call:{:?}", name)
    //                                 }
    //                             },
    //                             Expr::VarIdent(name) => {
    //                                 let v = self.get_var_type(name);
    //                                 *var_type = v.clone();
    //                                 self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
    //                             },
    //                             Expr::Operation { a, op, b } => {
    //                                 let v = self.expr_to_vartype(var_value.clone());
    //                                 *var_type = v.clone();
    //                                 self.scopes.last_mut().unwrap().insert(var_name.to_string(), v);
    //                             },
    //                             _ => {/* idc */}
    //                         }
    //                     },
    //                     _ => {
    //                         self.scopes.last_mut().unwrap().insert(var_name.to_string(), var_type.clone());
    //                     }
    //                 }
    //             }
    //             _ => {/*idc*/}
    //         }
    //     }
    //     self.ast = ast;
    //     &self.ast
    // }

    pub fn typecheck(&self) -> &Vec<TypedStmt> {
        &self.ast
    }
        
}
