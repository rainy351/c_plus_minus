#[derive(Debug, Clone, PartialEq)]
pub enum Ops {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Gt,
    Lt,
    Equals,
    NotEquals,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VarType {
    Idk,
    TypeString,
    TypeInt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    StringLit(String),
    VarIdent(String),
    Operation {
        a: Box<Expr>,
        op: Ops,
        b: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Include {
        lib_name:  String,
    },
    Variable {
        var_name:  String,
        var_type:  VarType,
        var_value: Expr,
    },
    Assignment {
        var_name:  String,
        new_value: Expr,
    },
    PressF {
        var_name:  String,
    },
    FnDecl {
        fn_name: String,
        params: Vec<(VarType, String)>,
        body: Vec<Stmt>,
        returns: VarType,
    },
    Return {
        value: Expr,
    },
    IfElse {
        condition: Expr,
        true_branch: Vec<Stmt>,
        false_branch: Option<Vec<Stmt>>,
    },
    Stonks {
        condition: Expr,
        stonks_branch: Vec<Stmt>,
        not_stonks_branch: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Expr(Expr),
}
