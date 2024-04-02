use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kw {
    Break,
    Case,
    Char,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Union,
    Unsigned,
    Void,
    While,
}

pub const KW_MAP: &[(Kw, &str)] = &[
    (Kw::Break, "break"),
    (Kw::Case, "case"),
    (Kw::Char, "char"),
    (Kw::Continue, "continue"),
    (Kw::Default, "default"),
    (Kw::Do, "do"),
    (Kw::Double, "double"),
    (Kw::Else, "else"),
    (Kw::Enum, "enum"),
    (Kw::Float, "float"),
    (Kw::For, "for"),
    (Kw::Goto, "goto"),
    (Kw::If, "if"),
    (Kw::Int, "int"),
    (Kw::Long, "long"),
    (Kw::Return, "return"),
    (Kw::Short, "short"),
    (Kw::Signed, "signed"),
    (Kw::Sizeof, "sizeof"),
    (Kw::Static, "static"),
    (Kw::Struct, "struct"),
    (Kw::Switch, "switch"),
    (Kw::Union, "union"),
    (Kw::Unsigned, "unsigned"),
    (Kw::Void, "void"),
    (Kw::While, "while"),
];

impl Kw {
    fn to_str(self) -> &'static str {
        KW_MAP
            .iter()
            .find(|(kw, _)| kw == &self)
            .expect(&format!("unexpected missing keyword: {self:?}"))
            .1
    }
}

impl Display for Kw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
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

pub const PUNCT_MAP: &[(Punct, &str)] = &[
    (Punct::LBrace, "{"),
    (Punct::RBrace, "}"),
    (Punct::LBrack, "["),
    (Punct::RBrack, "]"),
    (Punct::LParen, "("),
    (Punct::RParen, ")"),
    (Punct::Semi, ";"),
    (Punct::Colon, ":"),
    (Punct::Question, "?"),
    (Punct::Dot, "."),
    (Punct::Arrow, "->"),
    (Punct::Tilde, "~"),
    (Punct::Exclam, "!"),
    (Punct::Plus, "+"),
    (Punct::Dash, "-"),
    (Punct::Star, "*"),
    (Punct::Slash, "/"),
    (Punct::Percent, "%"),
    (Punct::Hat, "^"),
    (Punct::Amp, "&"),
    (Punct::Pipe, "|"),
    (Punct::Eq, "="),
    (Punct::PlusEq, "+="),
    (Punct::DashEq, "-="),
    (Punct::StarEq, "*="),
    (Punct::SlashEq, "/="),
    (Punct::PercentEq, "%="),
    (Punct::HatEq, "^="),
    (Punct::AmpEq, "&="),
    (Punct::PipeEq, "|="),
    (Punct::Eq2, "=="),
    (Punct::ExclamEq, "!="),
    (Punct::Lt, "<"),
    (Punct::Gt, ">"),
    (Punct::LtEq, "<="),
    (Punct::GtEq, ">="),
    (Punct::Amp2, "&&"),
    (Punct::Pipe2, "||"),
    (Punct::Lt2, "<<"),
    (Punct::Gt2, ">>"),
    (Punct::Lt2Eq, "<<="),
    (Punct::Gt2Eq, ">>="),
    (Punct::Plus2, "++"),
    (Punct::Dash2, "--"),
    (Punct::Comma, ","),
];

impl Punct {
    fn to_str(self) -> &'static str {
        PUNCT_MAP
            .iter()
            .find(|(punct, _)| punct == &self)
            .expect(&format!("unexpected missing punct {self:?}"))
            .1
    }
}

impl Display for Punct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(u64),
    Char(char),
    Float(f64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Lit(Lit),
    Kw(Kw),
    Punct(Punct),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Token::Ident(id) => {
                write!(f, "TOKEN IDENTIFIER {id:?}")
            }
            Token::Lit(Lit::Int(int)) => {
                write!(f, "TOKEN LITERAL INTEGER {int}")
            }
            Token::Lit(Lit::Char(char)) => {
                write!(f, "TOKEN LITERAL CHARACTER {char}")
            }
            Token::Lit(Lit::Float(float)) => {
                write!(f, "TOKEN LITERAL FLOAT {float}")
            }
            Token::Lit(Lit::Str(str)) => {
                write!(f, "TOKEN LITERAL STRING {str:?}")
            }
            Token::Kw(kw) => {
                write!(f, "TOKEN KEYWORD {kw}")
            }
            Token::Punct(punct) => {
                write!(f, "TOKEN PUNCTUATION {punct}")
            }
        }
    }
}
