use crate::common::{self, GenericModule, GenericPort, GenericStmt};
use crate::pretty::{PrettyPrinter, PRETTY_INDENT};
use pretty::RcDoc;
use std::fmt;

pub use common::EventTy;
pub use common::Expr;
pub use common::Id;

#[derive(Clone, Debug)]
pub enum Ty {
    Int,
    Width(u64),
}

impl Ty {
    pub fn width(&self) -> u64 {
        match self {
            Ty::Width(w) => w.clone(),
            _ => panic!("Error: type does not support width"),
        }
    }
}

impl PrettyPrinter for Ty {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Ty::Int => RcDoc::text("int"),
            Ty::Width(w) => match w {
                0 => panic!("Error: width must be greater than zero"),
                1 => RcDoc::nil(),
                n => RcDoc::text("[")
                    .append(RcDoc::as_string(n - 1))
                    .append(RcDoc::text(":"))
                    .append(RcDoc::text("0"))
                    .append(RcDoc::text("]")),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum Decl {
    Int(Id, Ty),
    Wire(Id, Ty),
    Reg(Id, Ty),
}

impl PrettyPrinter for Decl {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Decl::Int(name, ty) => ty
                .to_doc()
                .append(RcDoc::space())
                .append(RcDoc::as_string(name)),
            Decl::Wire(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("wire")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
            Decl::Reg(name, ty) => {
                let extra_space = match ty.width() {
                    1 => RcDoc::nil(),
                    _ => RcDoc::space(),
                };
                RcDoc::text("reg")
                    .append(RcDoc::space())
                    .append(ty.to_doc())
                    .append(extra_space)
                    .append(RcDoc::as_string(name))
            }
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Sequential {
    Wildcard,
    Event(EventTy, Expr),
    If(Expr, Vec<Sequential>, Vec<Sequential>),
}

impl PrettyPrinter for Sequential {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            // wildcard for sensitivity list
            Sequential::Wildcard => RcDoc::text("*"),
            Sequential::Event(ty, expr) => ty.to_doc().append(RcDoc::space()).append(expr.to_doc()),
            Sequential::If(expr, _, _) => RcDoc::text("if")
                .append(RcDoc::space())
                .append(RcDoc::text("("))
                .append(expr.to_doc())
                .append(RcDoc::text(")")),
        }
    }
}

impl fmt::Display for Sequential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

#[derive(Clone, Debug)]
pub enum Parallel {
    Assign,
    Always,
}

impl PrettyPrinter for Parallel {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Parallel::Assign => RcDoc::text("assign"),
            Parallel::Always => RcDoc::text("always"),
        }
    }
}

impl fmt::Display for Parallel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Stmt = GenericStmt<Decl, Parallel>;

impl PrettyPrinter for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Decl(decl) => decl.to_doc(),
            Stmt::Parallel(par) => par.to_doc(),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Port = GenericPort<Decl>;

impl PrettyPrinter for Port {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Port::Input(decl) => RcDoc::text("input")
                .append(RcDoc::space())
                .append(decl.to_doc()),
            Port::Output(decl) => RcDoc::text("output")
                .append(RcDoc::space())
                .append(decl.to_doc()),
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}

pub type Module = GenericModule<Decl, Parallel>;

impl Module {
    pub fn new_with_name(name: &str) -> Module {
        Module {
            name: name.to_string(),
            ports: Vec::new(),
            body: Vec::new(),
        }
    }
}

impl PrettyPrinter for Module {
    fn to_doc(&self) -> RcDoc<()> {
        let mut body_doc = RcDoc::nil();
        for decl in self.body.iter() {
            body_doc = body_doc
                .append(RcDoc::hardline())
                .append(decl.to_doc())
                .append(RcDoc::text(";"));
        }
        body_doc = body_doc.nest(PRETTY_INDENT).group();
        RcDoc::text("module")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::space())
            .append(RcDoc::text("("))
            .append(RcDoc::text(")"))
            .append(RcDoc::text(";"))
            .append(body_doc)
            .append(RcDoc::hardline())
            .append(RcDoc::text("endmodule"))
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty())
    }
}
