use crate::{indent, Lit, PrettyPrint, Punct};
use std::fmt::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum BasicTy {
    Void,
    Char,
    Int,
    UnsignedInt,
    Float,
    Double,
}

impl PrettyPrint for BasicTy {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        write!(w, "{}BasicTy ", indent(depth))?;
        match self {
            BasicTy::Void => write!(w, "Void")?,
            BasicTy::Char => write!(w, "Char")?,
            BasicTy::Int => write!(w, "Int")?,
            BasicTy::UnsignedInt => write!(w, "UnsignedInt")?,
            BasicTy::Float => write!(w, "Float")?,
            BasicTy::Double => write!(w, "Double")?,
        }
        writeln!(w)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PtrTy {
    pub ty: Box<Ty>,
}

impl PrettyPrint for PtrTy {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}PtrTy ty", indent(depth))?;
        self.ty.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayTy {
    pub ty: Box<Ty>,
    pub length: u64,
}

impl PrettyPrint for ArrayTy {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}ArrayTy length ty", indent(depth),)?;
        writeln!(w, "{}{}", indent(depth + 1), self.length)?;
        self.ty.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructTy {
    pub name: String,
}

impl PrettyPrint for StructTy {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}StructTy name", indent(depth))?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Basic(BasicTy),
    Ptr(PtrTy),
    Array(ArrayTy),
    Struct(StructTy),
}

impl PrettyPrint for Ty {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}Ty ty", indent(depth))?;
        match self {
            Ty::Basic(ty) => ty.pretty_fmt(w, depth + 1),
            Ty::Ptr(ty) => ty.pretty_fmt(w, depth + 1),
            Ty::Array(ty) => ty.pretty_fmt(w, depth + 1),
            Ty::Struct(ty) => ty.pretty_fmt(w, depth + 1),
        }
    }
}

/// An identifier expression
/// `myIdentifier`
#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub ident: String,
}

impl PrettyPrint for IdentExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}IdentExpr {:?}", indent(depth), self.ident)
    }
}

/// A constant literal expression
#[derive(Debug, Clone, PartialEq)]
pub struct ConstExpr {
    pub lit: Lit,
}

impl PrettyPrint for ConstExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}ConstExpr lit", indent(depth))?;
        self.lit.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Incr,
    Decr,
    Pos,
    Neg,
}

impl UnOp {
    fn to_punct(&self) -> Punct {
        match self {
            UnOp::Incr => Punct::Plus2,
            UnOp::Decr => Punct::Dash2,
            UnOp::Pos => Punct::Plus,
            UnOp::Neg => Punct::Dash,
        }
    }
}

impl PrettyPrint for UnOp {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}UnOp {}", indent(depth), self.to_punct().to_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnOpExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl PrettyPrint for UnOpExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}UnOpExpr op expr", indent(depth))?;
        self.op.pretty_fmt(w, depth + 1)?;
        self.expr.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
}

impl BinOp {
    fn to_punct(&self) -> Punct {
        match self {
            BinOp::Add => Punct::Plus,
            BinOp::Sub => Punct::Dash,
            BinOp::Mul => Punct::Star,
            BinOp::Div => Punct::Slash,
            BinOp::Eq => Punct::Eq2,
            BinOp::Ne => Punct::ExclamEq,
            BinOp::Lt => Punct::LtEq,
            BinOp::Gt => Punct::GtEq,
        }
    }
}

impl PrettyPrint for BinOp {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}BinOp {}", indent(depth), self.to_punct().to_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinOp,
}

impl PrettyPrint for BinOpExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}BinOpExpr op left right", indent(depth))?;
        self.op.pretty_fmt(w, depth + 1)?;
        self.left.pretty_fmt(w, depth + 1)?;
        self.right.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssnExpr {
    pub lvalue: Box<Expr>,
    pub expr: Box<Expr>,
}

impl PrettyPrint for AssnExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}AssnExpr lvalue expr", indent(depth))?;
        self.lvalue.pretty_fmt(w, depth + 1)?;
        self.expr.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub fun: Box<Expr>,
    pub params: Vec<Expr>,
}

impl PrettyPrint for CallExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(
            w,
            "{}CallExpr fun params[{}]",
            indent(depth),
            self.params.len()
        )?;
        self.fun.pretty_fmt(w, depth + 1)?;
        for param in &self.params {
            param.pretty_fmt(w, depth + 1)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub expr: Box<Expr>,
    pub field: String,
}

impl PrettyPrint for MemberExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}MemberExpr field expr", indent(depth))?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.field)?;
        self.expr.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerefExpr {
    pub expr: Box<Expr>,
}

impl PrettyPrint for DerefExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}DerefExpr expr", indent(depth))?;
        self.expr.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RefExpr {
    pub expr: Box<Expr>,
}

impl PrettyPrint for RefExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}RefExpr expr", indent(depth))?;
        self.expr.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
}

impl PrettyPrint for ParenExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}ParenExpr expr", indent(depth))?;
        self.expr.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommaExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl PrettyPrint for CommaExpr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}CommaExpr left right", indent(depth))?;
        self.left.pretty_fmt(w, depth + 1)?;
        self.right.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(IdentExpr),
    Const(ConstExpr),
    Assn(AssnExpr),
    BinOp(BinOpExpr),
    UnOp(UnOpExpr),
    Call(CallExpr),
    Member(MemberExpr),
    Deref(DerefExpr),
    Ref(RefExpr),
    Paren(ParenExpr),
    Comma(CommaExpr),
}

impl PrettyPrint for Expr {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        write!(w, "{}Expr ", indent(depth))?;
        match self {
            Expr::Ident(expr) => {
                write!(w, "Ident expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Const(expr) => {
                write!(w, "Const expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Assn(expr) => {
                write!(w, "Assn expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::BinOp(expr) => {
                write!(w, "BinOp expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::UnOp(expr) => {
                write!(w, "UnOp expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Call(expr) => {
                write!(w, "Call expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Member(expr) => {
                write!(w, "Member expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Deref(expr) => {
                write!(w, "Deref expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Ref(expr) => {
                write!(w, "Ref expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Paren(expr) => {
                write!(w, "Paren expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            Expr::Comma(expr) => {
                write!(w, "Comma expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclStmt {
    pub ty: Ty,
    pub ident: String,
}

impl PrettyPrint for DeclStmt {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}DeclStmt ty ident", indent(depth))?;
        self.ty.pretty_fmt(w, depth + 1)?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.ident)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

impl PrettyPrint for BlockStmt {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}BlockStmt stmts[{}]", indent(depth), self.stmts.len())?;
        for stmt in &self.stmts {
            stmt.pretty_fmt(w, depth + 1)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub cond: Expr,
    pub stmt: Box<Stmt>,
}

impl PrettyPrint for IfStmt {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}IfStmt cond stmt", indent(depth))?;
        self.cond.pretty_fmt(w, depth + 1)?;
        self.stmt.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseStmt {
    pub cond: Expr,
    pub stmt_true: Box<Stmt>,
    pub stmt_false: Box<Stmt>,
}

impl PrettyPrint for IfElseStmt {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}IfElseStmt cond stmt_true stmt_false", indent(depth))?;
        self.cond.pretty_fmt(w, depth + 1)?;
        self.stmt_true.pretty_fmt(w, depth + 1)?;
        self.stmt_false.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
}

impl PrettyPrint for ReturnStmt {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        write!(w, "{}ReturnStmt", indent(depth))?;
        match &self.expr {
            Some(expr) => {
                writeln!(w, " expr")?;
                expr.pretty_fmt(w, depth + 1)
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// The empty statement `;`
    Empty,
    Decl(DeclStmt),
    Block(BlockStmt),
    If(IfStmt),
    IfElse(IfElseStmt),
    Return(ReturnStmt),
}

impl PrettyPrint for Stmt {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        write!(w, "{}Stmt ", indent(depth))?;
        match self {
            Stmt::Empty => writeln!(w, "Empty"),
            Stmt::Decl(stmt) => {
                writeln!(w, "Decl stmt")?;
                stmt.pretty_fmt(w, depth + 1)
            }
            Stmt::Block(stmt) => {
                writeln!(w, "Block stmt")?;
                stmt.pretty_fmt(w, depth + 1)
            }
            Stmt::If(stmt) => {
                writeln!(w, "If stmt")?;
                stmt.pretty_fmt(w, depth + 1)
            }
            Stmt::IfElse(stmt) => {
                writeln!(w, "IfElse stmt")?;
                stmt.pretty_fmt(w, depth + 1)
            }
            Stmt::Return(stmt) => {
                writeln!(w, "Return stmt")?;
                stmt.pretty_fmt(w, depth + 1)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub ident: String,
    pub ty: Ty,
}

impl PrettyPrint for Param {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}Param ident ty", indent(depth))?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.ident)?;
        self.ty.pretty_fmt(w, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDefn {
    pub name: String,
    pub ret: Ty,
    pub params: Vec<Param>,
    pub stmts: Vec<Stmt>,
}

impl PrettyPrint for FnDefn {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(
            w,
            "{}FnDefn name ret param[{}] stmt[{}]",
            indent(depth),
            self.params.len(),
            self.stmts.len()
        )?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.name)?;
        self.ret.pretty_fmt(w, depth + 1)?;
        for param in &self.params {
            param.pretty_fmt(w, depth + 1)?;
        }
        for stmt in &self.stmts {
            stmt.pretty_fmt(w, depth + 1)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub ty: Ty,
    pub ident: String,
}

impl PrettyPrint for StructField {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}StructField ty ident", indent(depth))?;
        self.ty.pretty_fmt(w, depth + 1)?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.ident)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDefn {
    pub name: String,
    pub fields: Vec<StructField>,
}

impl PrettyPrint for StructDefn {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(
            w,
            "{}StructDefn name field[{}]",
            indent(depth),
            self.fields.len()
        )?;
        writeln!(w, "{}{:?}", indent(depth + 1), self.name)?;
        for field in &self.fields {
            field.pretty_fmt(w, depth + 1)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FnDefn(FnDefn),
    StructDefn(StructDefn),
}

impl PrettyPrint for Item {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        write!(w, "{}Item ", indent(depth))?;
        match self {
            Item::FnDefn(fn_defn) => {
                writeln!(w, "FnDefn fn_defn")?;
                fn_defn.pretty_fmt(w, depth + 1)
            }
            Item::StructDefn(struct_defn) => {
                writeln!(w, "StructDefn struct_defn")?;
                struct_defn.pretty_fmt(w, depth + 1)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

impl PrettyPrint for Program {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}Program item[{}]", indent(depth), self.items.len())?;
        for item in &self.items {
            item.pretty_fmt(w, depth + 1)?;
        }
        Ok(())
    }
}
