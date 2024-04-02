use crate::{ArrayTy, BasicTy, Kw, Lit, Program, PtrTy, Punct, StructTy, Token, Ty};
use anyhow::{anyhow, bail, Result};

#[derive(Clone)]
struct TokenCursor<'a> {
    idx: usize,
    tokens: &'a [Token],
}

impl<'a> TokenCursor<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        TokenCursor { idx: 0, tokens }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }

    pub fn npeek(&self) -> &[Token] {
        &self.tokens[self.idx..]
    }

    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }

    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.idx);
        if self.idx < self.tokens.len() {
            self.idx += 1;
        }
        token
    }

    pub fn advance(&mut self, amount: usize) {
        if self.idx + amount > self.tokens.len() {
            panic!("advanced past end of cursor");
        }
        self.idx += amount;
    }

    pub fn apply(&mut self, cursor: TokenCursor<'a>) {
        let _ = std::mem::replace(self, cursor);
    }
}

/// Parse a typed identifier, returning both the type and the identifier portion.
/// Typed identifiers seem to be a common pattern throughout C, used in
/// variable declarations, parameter declarations, etc.
///
/// e.g. `struct my_struct ptr[10]` => length 10 array of struct my_struct
fn parse_typed_ident<'a>(cursor: &mut TokenCursor<'a>) -> Result<(Ty, String)> {
    let mut new_cursor = cursor.clone();

    let mut ty: Option<Ty> = None;
    let mut ident: Option<String> = None;

    // Munch type prefix
    match new_cursor.npeek() {
        &[Token::Kw(Kw::Void), ..] => {
            ty = Some(Ty::Basic(BasicTy::Void));
            new_cursor.advance(1);
        }
        &[Token::Kw(Kw::Int), ..] => {
            ty = Some(Ty::Basic(BasicTy::Int));
            new_cursor.advance(1);
        }
        &[Token::Kw(Kw::Unsigned), Token::Kw(Kw::Int), ..] => {
            ty = Some(Ty::Basic(BasicTy::UnsignedInt));
            new_cursor.advance(2);
        }
        &[Token::Kw(Kw::Char), ..] => {
            ty = Some(Ty::Basic(BasicTy::Char));
            new_cursor.advance(1);
        }
        &[Token::Kw(Kw::Struct), Token::Ident(ref id), ..] => {
            ty = Some(Ty::Struct(StructTy { name: id.clone() }));
            new_cursor.advance(2);
        }
        _ => bail!("expected type"),
    }

    // Munch pointer
    loop {
        match new_cursor.peek() {
            Some(&Token::Punct(Punct::Star)) => {
                ty = Some(Ty::Ptr(PtrTy {
                    ty: Box::new(ty.unwrap()),
                }));
                new_cursor.advance(1);
            }
            _ => break,
        }
    }

    // Munch identifier
    match new_cursor.peek() {
        Some(&Token::Ident(ref id)) => {
            ident = Some(id.clone());
            new_cursor.advance(1);
        }
        _ => bail!("expected identifier"),
    }

    // Munch array brackets
    match new_cursor.peek() {
        Some(Token::Punct(Punct::LBrack)) => {
            new_cursor.advance(1);
            match new_cursor.npeek() {
                &[Token::Lit(Lit::Int(num)), Token::Punct(Punct::RBrack), ..] => {
                    let inner_ty = ty.unwrap();
                    if let Ty::Basic(BasicTy::Void) = inner_ty {
                        bail!("cannot have array of void");
                    }
                    ty = Some(Ty::Array(ArrayTy {
                        ty: Box::new(inner_ty),
                        length: num,
                    }));
                    new_cursor.advance(2);
                }
                &[Token::Punct(Punct::RBrack), ..] => {
                    bail!("array with no type currently not supported");
                }
                _ => {}
            }
        }
        _ => {}
    }

    cursor.apply(new_cursor);
    match (ty, ident) {
        (Some(ty), Some(ident)) => Ok((ty, ident)),
        _ => unreachable!(),
    }
}

/// Parse the token sequence into an AST tree for a C program.
pub fn parse(tokens: Vec<Token>) -> Program {
    let mut cursor = TokenCursor::new(&tokens);
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{lex, ArrayTy, BasicTy, PtrTy, StructTy, Ty};

    use super::{parse_typed_ident, TokenCursor};

    #[test]
    fn test_parse_typed_ident() {
        let test_cases = [
            ("int a", Some((Ty::Basic(BasicTy::Int), String::from("a")))),
            (
                "unsigned int bebop_123",
                Some((Ty::Basic(BasicTy::UnsignedInt), String::from("bebop_123"))),
            ),
            (
                "struct my_struct beep",
                Some((
                    Ty::Struct(StructTy {
                        name: String::from("my_struct"),
                    }),
                    String::from("beep"),
                )),
            ),
            (
                "void *ptr",
                Some((
                    Ty::Ptr(PtrTy {
                        ty: Box::new(Ty::Basic(BasicTy::Void)),
                    }),
                    String::from("ptr"),
                )),
            ),
            (
                "char ptr[20]",
                Some((
                    Ty::Array(ArrayTy {
                        ty: Box::new(Ty::Basic(BasicTy::Char)),
                        length: 20,
                    }),
                    String::from("ptr"),
                )),
            ),
            ("void ptr[100]", None),
            ("int ptr[]", None),
            ("", None),
        ];
        for (input, expect) in test_cases {
            let tokens = lex(input).unwrap();
            let mut cursor = TokenCursor::new(&tokens);
            let output = parse_typed_ident(&mut cursor);
            assert_eq!(output.ok(), expect);
        }
    }
}
