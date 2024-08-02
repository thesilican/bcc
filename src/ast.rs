use std::fmt::{self, Display, Formatter, Write};

use crate::Lit;

const INDENT: &str = "  ";

trait PrettyPrint {
    fn pretty_fmt(&self, writer: &mut impl Write, depth: usize) -> fmt::Result;
    fn pretty_print(&self) -> String {
        let mut buffer = String::new();
        self.pretty_fmt(&mut buffer, 0).unwrap();
        buffer
    }
}

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
    fn pretty_fmt(&self, writer: &mut impl Write, depth: usize) -> fmt::Result {
        write!(writer, "{}BasicTy ", INDENT.repeat(depth))?;
        match self {
            BasicTy::Void => write!(writer, "Void")?,
            BasicTy::Char => write!(writer, "Char")?,
            BasicTy::Int => write!(writer, "Int")?,
            BasicTy::UnsignedInt => write!(writer, "UnsignedInt")?,
            BasicTy::Float => write!(writer, "Float")?,
            BasicTy::Double => write!(writer, "Double")?,
        }
        writeln!(writer)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PtrTy {
    pub ty: Box<Ty>,
}

impl PrettyPrint for PtrTy {
    fn pretty_fmt(&self, writer: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(writer, "{}PtrTy <ty>", INDENT.repeat(depth))?;
        self.ty.pretty_fmt(writer, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayTy {
    pub ty: Box<Ty>,
    pub length: u64,
}

impl PrettyPrint for ArrayTy {
    fn pretty_fmt(&self, writer: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(
            writer,
            "{}ArrayTy <ty> {}",
            INDENT.repeat(depth),
            self.length
        )?;
        self.ty.pretty_fmt(writer, depth + 1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructTy {
    pub name: String,
}

impl PrettyPrint for StructTy {
    fn pretty_fmt(&self, writer: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(writer, "{}StructTy {:?}", INDENT.repeat(depth), self.name)
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
    fn pretty_fmt(&self, writer: &mut impl Write, depth: usize) -> fmt::Result {
        writeln!(writer, "{}Ty <ty>", INDENT.repeat(depth))?;
        match self {
            Ty::Basic(ty) => ty.pretty_fmt(writer, depth + 1),
            Ty::Ptr(ty) => ty.pretty_fmt(writer, depth + 1),
            Ty::Array(ty) => ty.pretty_fmt(writer, depth + 1),
            Ty::Struct(ty) => ty.pretty_fmt(writer, depth + 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstExpr {
    pub ty: Ty,
    pub lit: Lit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Incr,
    Decr,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnOpExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinOp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssnExpr {
    pub lvalue: Box<Expr>,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub fun: Box<Expr>,
    pub params: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub expr: Box<Expr>,
    pub field: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerefExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RefExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommaExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct DeclStmt {
    pub ty: Ty,
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub cond: Expr,
    pub stmt: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseStmt {
    pub cond: Expr,
    pub stmt_true: Box<Stmt>,
    pub stmt_false: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub expr: Expr,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub ident: String,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDefn {
    pub name: String,
    pub ret: Ty,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub ty: Ty,
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDefn {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FnDefn(FnDefn),
    StructDefn(StructDefn),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}
