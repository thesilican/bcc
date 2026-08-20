use crate::{indent, PrettyPrint};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kw {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

impl Kw {
    pub const ALL: &[Kw] = &[
        Kw::Auto,
        Kw::Break,
        Kw::Case,
        Kw::Char,
        Kw::Const,
        Kw::Continue,
        Kw::Default,
        Kw::Do,
        Kw::Double,
        Kw::Else,
        Kw::Enum,
        Kw::Extern,
        Kw::Float,
        Kw::For,
        Kw::Goto,
        Kw::If,
        Kw::Inline,
        Kw::Int,
        Kw::Long,
        Kw::Register,
        Kw::Restrict,
        Kw::Return,
        Kw::Short,
        Kw::Signed,
        Kw::Sizeof,
        Kw::Static,
        Kw::Struct,
        Kw::Switch,
        Kw::Typedef,
        Kw::Union,
        Kw::Unsigned,
        Kw::Void,
        Kw::Volatile,
        Kw::While,
    ];

    pub fn to_str(self) -> &'static str {
        match self {
            Kw::Auto => "auto",
            Kw::Break => "break",
            Kw::Case => "case",
            Kw::Char => "char",
            Kw::Const => "const",
            Kw::Continue => "continue",
            Kw::Default => "default",
            Kw::Do => "do",
            Kw::Double => "double",
            Kw::Else => "else",
            Kw::Enum => "enum",
            Kw::Extern => "extern",
            Kw::Float => "float",
            Kw::For => "for",
            Kw::Goto => "goto",
            Kw::If => "if",
            Kw::Inline => "inline",
            Kw::Int => "int",
            Kw::Long => "long",
            Kw::Register => "register",
            Kw::Restrict => "restrict",
            Kw::Return => "return",
            Kw::Short => "short",
            Kw::Signed => "signed",
            Kw::Sizeof => "sizeof",
            Kw::Static => "static",
            Kw::Struct => "struct",
            Kw::Switch => "switch",
            Kw::Typedef => "typedef",
            Kw::Union => "union",
            Kw::Unsigned => "unsigned",
            Kw::Void => "void",
            Kw::Volatile => "volatile",
            Kw::While => "while",
        }
    }
}

impl PrettyPrint for Kw {
    fn pretty_fmt(&self, w: &mut impl fmt::Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}Kw {}", indent(depth), self.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punct {
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    LParen,
    RParen,
    Semi,
    Colon,
    Question,
    Dot,
    Arrow,
    Tilde,
    Exclam,
    Plus,
    Dash,
    Star,
    Slash,
    Percent,
    Hat,
    Amp,
    Pipe,
    Eq,
    PlusEq,
    DashEq,
    StarEq,
    SlashEq,
    PercentEq,
    HatEq,
    AmpEq,
    PipeEq,
    Eq2,
    ExclamEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Amp2,
    Pipe2,
    Lt2,
    Gt2,
    Lt2Eq,
    Gt2Eq,
    Plus2,
    Dash2,
    Comma,
}

impl Punct {
    pub const ALL: &[Punct] = &[
        Punct::LBrace,
        Punct::RBrace,
        Punct::LBrack,
        Punct::RBrack,
        Punct::LParen,
        Punct::RParen,
        Punct::Semi,
        Punct::Colon,
        Punct::Question,
        Punct::Dot,
        Punct::Arrow,
        Punct::Tilde,
        Punct::Exclam,
        Punct::Plus,
        Punct::Dash,
        Punct::Star,
        Punct::Slash,
        Punct::Percent,
        Punct::Hat,
        Punct::Amp,
        Punct::Pipe,
        Punct::Eq,
        Punct::PlusEq,
        Punct::DashEq,
        Punct::StarEq,
        Punct::SlashEq,
        Punct::PercentEq,
        Punct::HatEq,
        Punct::AmpEq,
        Punct::PipeEq,
        Punct::Eq2,
        Punct::ExclamEq,
        Punct::Lt,
        Punct::Gt,
        Punct::LtEq,
        Punct::GtEq,
        Punct::Amp2,
        Punct::Pipe2,
        Punct::Lt2,
        Punct::Gt2,
        Punct::Lt2Eq,
        Punct::Gt2Eq,
        Punct::Plus2,
        Punct::Dash2,
        Punct::Comma,
    ];

    pub fn to_str(self) -> &'static str {
        match self {
            Punct::LBrace => "{",
            Punct::RBrace => "}",
            Punct::LBrack => "[",
            Punct::RBrack => "]",
            Punct::LParen => "(",
            Punct::RParen => ")",
            Punct::Semi => ";",
            Punct::Colon => ":",
            Punct::Question => "?",
            Punct::Dot => ".",
            Punct::Arrow => "->",
            Punct::Tilde => "~",
            Punct::Exclam => "!",
            Punct::Plus => "+",
            Punct::Dash => "-",
            Punct::Star => "*",
            Punct::Slash => "/",
            Punct::Percent => "%",
            Punct::Hat => "^",
            Punct::Amp => "&",
            Punct::Pipe => "|",
            Punct::Eq => "=",
            Punct::PlusEq => "+=",
            Punct::DashEq => "-=",
            Punct::StarEq => "*=",
            Punct::SlashEq => "/=",
            Punct::PercentEq => "%=",
            Punct::HatEq => "^=",
            Punct::AmpEq => "&=",
            Punct::PipeEq => "|=",
            Punct::Eq2 => "==",
            Punct::ExclamEq => "!=",
            Punct::Lt => "<",
            Punct::Gt => ">",
            Punct::LtEq => "<=",
            Punct::GtEq => ">=",
            Punct::Amp2 => "&&",
            Punct::Pipe2 => "||",
            Punct::Lt2 => "<<",
            Punct::Gt2 => ">>",
            Punct::Lt2Eq => "<<=",
            Punct::Gt2Eq => ">>=",
            Punct::Plus2 => "++",
            Punct::Dash2 => "--",
            Punct::Comma => ",",
        }
    }
}

impl PrettyPrint for Punct {
    fn pretty_fmt(&self, w: &mut impl fmt::Write, depth: usize) -> fmt::Result {
        writeln!(w, "{}Punct {}", indent(depth), self.to_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(u64),
    Char(char),
    Float(f64),
    Str(String),
}

impl PrettyPrint for Lit {
    fn pretty_fmt(&self, w: &mut impl fmt::Write, depth: usize) -> fmt::Result {
        write!(w, "{}Lit ", indent(depth))?;
        match self {
            Lit::Int(val) => writeln!(w, "Int {}", val),
            Lit::Char(val) => writeln!(w, "Char {}", val),
            Lit::Float(val) => writeln!(w, "Float {}", val),
            Lit::Str(val) => writeln!(w, "Str {}", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Lit(Lit),
    Kw(Kw),
    Punct(Punct),
}

impl PrettyPrint for Token {
    fn pretty_fmt(&self, w: &mut impl fmt::Write, depth: usize) -> fmt::Result {
        write!(w, "{}Token ", indent(depth))?;
        match self {
            Token::Ident(ident) => write!(w, "Ident {:?}", ident),
            Token::Lit(Lit::Int(val)) => write!(w, "Lit Int {val}"),
            Token::Lit(Lit::Char(val)) => write!(w, "Lit Char {val:?}"),
            Token::Lit(Lit::Float(val)) => write!(w, "Lit Float {val}"),
            Token::Lit(Lit::Str(val)) => write!(w, "Lit Str {val:?}"),
            Token::Kw(kw) => write!(w, "Kw {}", kw.to_str()),
            Token::Punct(punct) => write!(w, "Punct {}", punct.to_str()),
        }
    }
}
