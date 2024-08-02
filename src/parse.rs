use crate::{
    ArrayTy, BasicTy, FnDefn, Item, Kw, Lit, Param, Program, PtrTy, Punct, Stmt, StructTy, Token,
    Ty,
};
use anyhow::{bail, Context, Result};

/// Allows a parser to consume a stream of tokens
#[derive(Clone)]
struct TokenCursor<'a> {
    idx: usize,
    tokens: &'a [Token],
}

impl<'a> TokenCursor<'a> {
    /// Create a new token cursor
    pub fn new(tokens: &'a [Token]) -> Self {
        TokenCursor { idx: 0, tokens }
    }

    /// Returns the first unconsumed token
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }

    //// Returns the remaining unconsumed tokens
    pub fn npeek(&self) -> &[Token] {
        &self.tokens[self.idx..]
    }

    /// Returns whether there are any remaining unconsumed tokens
    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }

    /// Return the next unconsumed token and advance the cursor by 1
    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.idx);
        if self.idx < self.tokens.len() {
            self.idx += 1;
        }
        token
    }

    /// Advance the cursor by 1. Will panic if advanced past the end of the stream, so
    /// you must ensure that there are tokens to consume. This function is usually paired
    /// with `TokenCursor::peek` or `TokenCursor::npeek`.
    pub fn advance(&mut self, amount: usize) {
        if self.idx + amount > self.tokens.len() {
            panic!("advanced past end of cursor");
        }
        self.idx += amount;
    }

    /// Replace the current cursor with another cursor, usually used to commit a "hypothetical" cursor
    pub fn replace(&mut self, cursor: TokenCursor<'a>) {
        let _ = std::mem::replace(self, cursor);
    }
}

/// Parse a typed identifier, returning both the type and the identifier portion.
/// Typed identifiers seem to be a common pattern throughout C, used in
/// variable declarations, parameter declarations, etc.
///
/// e.g. `struct my_struct ptr[10]` => length 10 array of struct my_struct
fn munch_typed_ident<'a>(cursor: &mut TokenCursor<'a>) -> Result<(Ty, String)> {
    let mut new_cursor = cursor.clone();

    let mut ty: Option<Ty>;
    let ident: Option<String>;

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

    cursor.replace(new_cursor);
    match (ty, ident) {
        (Some(ty), Some(ident)) => Ok((ty, ident)),
        _ => unreachable!(),
    }
}

/// Parse the empty statement `;`
fn munch_empty_stmt(cursor: &mut TokenCursor<'_>) -> Result<()> {
    if let Some(Token::Punct(Punct::Semi)) = cursor.peek() {
        cursor.advance(1);
        Ok(())
    } else {
        bail!("expected `;`");
    }
}

/// Parse a statement
fn munch_stmt(cursor: &mut TokenCursor<'_>) -> Result<Stmt> {
    // Try to parse empty statement
    if let Ok(()) = munch_empty_stmt(cursor) {
        return Ok(Stmt::Empty);
    }
    bail!("not implemented")
}

/// Parse a function definition
/// e.g. `int main() { /* */ }`
fn munch_fn_defn(cursor: &mut TokenCursor<'_>) -> Result<FnDefn> {
    let mut new_cursor = cursor.clone();
    let (ret, name) = munch_typed_ident(&mut new_cursor).context("expected typed identifier")?;
    match new_cursor.next() {
        Some(Token::Punct(Punct::LParen)) => {}
        _ => bail!("expected `(`"),
    }
    let mut params = Vec::new();
    // Special case: void parameter list
    if let &[Token::Kw(Kw::Void), Token::Punct(Punct::RParen), ..] = new_cursor.npeek() {
        new_cursor.advance(2);
    } else {
        loop {
            if let Ok((ty, ident)) = munch_typed_ident(&mut new_cursor) {
                let param = Param { ident, ty };
                params.push(param);
                match new_cursor.next() {
                    Some(Token::Punct(Punct::Comma)) => continue,
                    Some(Token::Punct(Punct::RParen)) => break,
                    _ => bail!("expected `,` or `)`"),
                }
            }
            if let Some(Token::Punct(Punct::RParen)) = new_cursor.peek() {
                new_cursor.advance(1);
                break;
            }
            bail!("expected parameter or `)`");
        }
    }
    match new_cursor.next() {
        Some(Token::Punct(Punct::LBrace)) => {}
        _ => bail!("expected `{{`"),
    }
    let mut body = Vec::new();
    loop {
        if let Ok(stmt) = munch_stmt(&mut new_cursor) {
            body.push(stmt);
            continue;
        }
        if let Some(Token::Punct(Punct::RBrace)) = new_cursor.peek() {
            new_cursor.advance(1);
            break;
        }
        bail!("expected `}}`");
    }

    cursor.replace(new_cursor);
    Ok(FnDefn {
        name,
        ret,
        params,
        body,
    })
}

/// Parse the token sequence into an AST tree for a C program.
pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut cursor = TokenCursor::new(&tokens);
    let mut items = Vec::new();
    loop {
        if let Ok(fn_defn) = munch_fn_defn(&mut cursor) {
            items.push(Item::FnDefn(fn_defn));
            continue;
        }
        if cursor.is_empty() {
            break;
        }
        bail!("expected EoF");
    }
    Ok(Program { items })
}

#[cfg(test)]
mod tests {
    use crate::{lex, ArrayTy, BasicTy, FnDefn, Param, PtrTy, StructTy, Ty};

    use super::{munch_fn_defn, munch_typed_ident, TokenCursor};

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
            let output = munch_typed_ident(&mut cursor);
            assert_eq!(output.ok(), expect);
        }
    }

    #[test]
    fn test_parse_fn_defn() {
        let test_cases = [
            (
                "int main(void)\n{}\n",
                Some(FnDefn {
                    name: String::from("main"),
                    ret: Ty::Basic(BasicTy::Int),
                    params: Vec::new(),
                    body: Vec::new(),
                }),
            ),
            (
                "int main(char a, char b) {}",
                Some(FnDefn {
                    name: String::from("main"),
                    ret: Ty::Basic(BasicTy::Int),
                    params: vec![
                        Param {
                            ident: String::from("a"),
                            ty: Ty::Basic(BasicTy::Char),
                        },
                        Param {
                            ident: String::from("b"),
                            ty: Ty::Basic(BasicTy::Char),
                        },
                    ],
                    body: Vec::new(),
                }),
            ),
            (
                "struct my_struct *my_fn(struct my_struct *my_param) {}",
                Some(FnDefn {
                    name: String::from("my_fn"),
                    ret: Ty::Ptr(PtrTy {
                        ty: Box::new(Ty::Struct(StructTy {
                            name: String::from("my_struct"),
                        })),
                    }),
                    params: vec![Param {
                        ident: String::from("my_param"),
                        ty: Ty::Ptr(PtrTy {
                            ty: Box::new(Ty::Struct(StructTy {
                                name: String::from("my_struct"),
                            })),
                        }),
                    }],
                    body: Vec::new(),
                }),
            ),
            ("int main ()", None),
        ];
        for (input, expect) in test_cases {
            let tokens = lex(input).unwrap();
            let mut cursor = TokenCursor::new(&tokens);
            let output = munch_fn_defn(&mut cursor);
            assert_eq!(output.ok(), expect);
        }
    }
}
