use crate::Lit;

pub trait AstNode {
    fn print(&self, depth: u32);
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

#[derive(Debug, Clone, PartialEq)]
pub struct PtrTy {
    pub ty: Box<Ty>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayTy {
    pub ty: Box<Ty>,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructTy {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Basic(BasicTy),
    Ptr(PtrTy),
    Array(ArrayTy),
    Struct(StructTy),
}

#[derive(Clone, PartialEq)]
pub struct IdentExpr {
    pub ident: String,
}

#[derive(Clone, PartialEq)]
pub struct ConstExpr {
    pub ty: Ty,
    pub lit: Lit,
}

#[derive(Clone, PartialEq)]
pub enum UnOp {
    Incr,
    Decr,
    Neg,
}

#[derive(Clone, PartialEq)]
pub struct UnOpExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinOp,
}

#[derive(Clone, PartialEq)]
pub struct AssnExpr {
    pub lvalue: Box<Expr>,
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct CallExpr {
    pub fun: Box<Expr>,
    pub params: Vec<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct MemberExpr {
    pub expr: Box<Expr>,
    pub field: String,
}

#[derive(Clone, PartialEq)]
pub struct DerefExpr {
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct RefExpr {
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct CommaExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Clone, PartialEq)]
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

pub struct DeclStmt {
    pub ty: Ty,
    pub ident: String,
}

pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

pub struct IfStmt {
    pub cond: Expr,
    pub stmt: Box<BlockStmt>,
}

pub struct IfElseStmt {
    pub cond: Expr,
    pub stmt_true: Box<BlockStmt>,
    pub stmt_false: Box<BlockStmt>,
}

pub struct ReturnStmt {
    pub expr: Expr,
}

pub enum Stmt {
    Decl(DeclStmt),
    Block(BlockStmt),
    If(IfStmt),
    IfElse(IfElseStmt),
    Return(ReturnStmt),
}

pub struct Param {
    pub ident: String,
    pub ty: Ty,
}

pub struct FnDefn {
    pub ret: Ty,
    pub params: Vec<Param>,
    pub body: BlockStmt,
}

pub struct StructEntry {
    pub ty: Ty,
    pub ident: String,
}

pub struct StructDefn {
    pub name: String,
    pub entries: Vec<StructEntry>,
}

pub enum Item {
    FnDefn(FnDefn),
    StructDefn(StructDefn),
}

pub struct Program {
    pub items: Vec<Item>,
}
