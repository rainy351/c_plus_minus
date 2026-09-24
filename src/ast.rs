#[derive(Debug, Clone, PartialEq)]
pub enum Ops {
    Plus,
    Minus,
    Mult,
    Div
}

#[derive(Debug, Clone, PartialEq)]
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
    Expr(Expr),
}
