use crate::pretty::PrettyPrinter;
use pretty::RcDoc;
use std::fmt;
use std::rc::Rc;

pub type Id = String;

pub type Width = u64;

impl PrettyPrinter for Width {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            0 => panic!("Error: width must be greater than zero"),
            1 => RcDoc::nil(),
            n => RcDoc::text("[")
                .append(RcDoc::as_string(n - 1))
                .append(RcDoc::text(":"))
                .append(RcDoc::text("0"))
                .append(RcDoc::text("]")),
        }
    }
}

// Reduce ops
#[derive(Clone, Debug)]
pub enum RedOp {
    LogNot,
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

impl PrettyPrinter for RedOp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            RedOp::LogNot => RcDoc::text("!"),
            RedOp::Not => RcDoc::text("~"),
            RedOp::And => RcDoc::text("&"),
            RedOp::Nand => RcDoc::text("~&"),
            RedOp::Or => RcDoc::text("|"),
            RedOp::Nor => RcDoc::text("~|"),
            RedOp::Xor => RcDoc::text("^"),
            RedOp::Xnor => RcDoc::text("~^"),
        }
    }
}

impl fmt::Display for RedOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Binop {
    Add,
}

impl PrettyPrinter for Binop {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Binop::Add => RcDoc::text("+"),
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(Id),
    Unop(RedOp, Rc<Expr>),
    Binop(Binop, Rc<Expr>, Rc<Expr>),
}

impl PrettyPrinter for Expr {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expr::Ref(name) => RcDoc::as_string(name),
            Expr::Unop(op, input) => op.to_doc().append(input.to_doc()),
            Expr::Binop(op, lhs, rhs) => lhs.to_doc()
                .append(RcDoc::space())
                .append(op.to_doc())
                .append(RcDoc::space())
                .append(rhs.to_doc()),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum GenericStmt<T, U> {
    Decl(T),
    Par(U),
}

#[derive(Clone, Debug)]
pub struct GenericModule<T, U> {
    pub stmt: Vec<GenericStmt<T, U>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unop_lognot() {
        assert_eq!("!".to_string(), RedOp::LogNot.to_string());
    }

    #[test]
    fn test_expr_binop_add_two_ref() {
        assert_eq!("a + b".to_string(), Expr::Binop(Binop::Add, Rc::new(Expr::Ref("a".to_string())), Rc::new(Expr::Ref("b".to_string()))).to_string());
    }
}
