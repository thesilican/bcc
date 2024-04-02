use crate::{Lit, Token, KW_MAP, PUNCT_MAP};
use anyhow::{bail, Result};
use std::str::Chars;

#[derive(Clone)]
struct Cursor<'a> {
    idx: usize,
    top: Option<char>,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let top = chars.next();
        Cursor { idx: 0, top, chars }
    }
    pub fn is_empty(&self) -> bool {
        self.top == None
    }
    pub fn peek(&self) -> Option<char> {
        self.top
    }
    pub fn next(&mut self) -> Option<char> {
        let output = self.top;
        self.top = self.chars.next();
        self.idx += 1;
        output
    }
    pub fn advance(&mut self, count: usize) {
        for _ in 0..count {
            if self.next().is_none() {
                panic!("advanced past end of cursor")
            }
        }
    }
    pub fn begins_with(&self, other: &str) -> Option<usize> {
        let mut chars = self.clone();
        let mut expect = other.chars();
        let mut count = 0;
        loop {
            match (chars.next(), expect.next()) {
                (None, None) => break,
                (Some(_), None) => break,
                (None, Some(_)) => return None,
                (Some(a), Some(b)) if a != b => return None,
                (Some(_), Some(_)) => {}
            }
            count += 1;
        }
        Some(count)
    }
}

fn munch_whitespace<'a>(cursor: &mut Cursor<'a>) -> Option<()> {
    let mut len = 0;
    loop {
        let top = cursor.peek();
        match top {
            Some(char) if char.is_ascii_whitespace() => {
                len += 1;
                cursor.advance(1);
            }
            _ => break,
        }
    }
    if len == 0 {
        None
    } else {
        Some(())
    }
}

fn munch_keyword<'a>(cursor: &mut Cursor<'a>) -> Option<Token> {
    let mut best_keyword = None;
    let mut best_len = 0;
    for &(keyword, text) in KW_MAP {
        match cursor.begins_with(text) {
            Some(len) if len > best_len => {
                best_keyword = Some(keyword);
                best_len = len;
            }
            _ => {}
        }
    }

    match best_keyword {
        Some(keyword) => {
            cursor.advance(best_len);
            Some(Token::Kw(keyword))
        }
        None => None,
    }
}

fn munch_punctuation<'a>(cursor: &mut Cursor<'a>) -> Option<Token> {
    let mut best_punct = None;
    let mut best_len = 0;
    for &(punct, text) in PUNCT_MAP {
        match cursor.begins_with(text) {
            Some(len) if len > best_len => {
                best_punct = Some(punct);
                best_len = len;
            }
            _ => {}
        }
    }

    match best_punct {
        Some(punct) => {
            cursor.advance(best_len);
            Some(Token::Punct(punct))
        }
        None => None,
    }
}

fn munch_identifier<'a>(cursor: &mut Cursor<'a>) -> Option<Token> {
    let mut ident = String::new();
    match cursor.peek() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            ident.push(c);
            cursor.advance(1);
        }
        _ => return None,
    }
    loop {
        match cursor.peek() {
            Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                ident.push(c);
                cursor.advance(1);
            }
            _ => break,
        }
    }
    let token = Token::Ident(ident);
    Some(token)
}

fn munch_string_char(cursor: &mut Cursor) -> Option<char> {
    let mut new_cursor = cursor.clone();
    match new_cursor.next() {
        Some('\\') => match new_cursor.next() {
            Some('n') => {
                cursor.advance(2);
                Some('\n')
            }
            Some('t') => {
                cursor.advance(2);
                Some('\t')
            }
            _ => None,
        },
        Some(c) => {
            cursor.advance(1);
            Some(c)
        }
        _ => None,
    }
}

fn munch_literal_string<'a>(cursor: &mut Cursor<'a>) -> Option<Token> {
    match cursor.peek() {
        Some(c) if c == '"' => {}
        _ => return None,
    }

    // Make a copy of the cursor so that we can rollback
    let mut new_cursor = cursor.clone();
    new_cursor.advance(1);
    let mut chars = String::new();
    loop {
        if new_cursor.peek() == Some('"') {
            new_cursor.advance(1);
            break;
        }
        match munch_string_char(&mut new_cursor) {
            Some(c) => chars.push(c),
            None => return None,
        }
    }
    std::mem::swap(cursor, &mut new_cursor);

    let token = Token::Lit(Lit::Str(chars));
    Some(token)
}

fn munch_literal_integer<'a>(cursor: &mut Cursor<'a>) -> Option<Token> {
    match cursor.peek() {
        Some(c) if ('0'..='9').contains(&c) => {}
        _ => return None,
    }

    let mut value = 0;
    loop {
        match cursor.peek() {
            Some(c) if ('0'..='9').contains(&c) => {
                let digit = c as u64 - '0' as u64;
                value = value * 10 + digit;
                cursor.advance(1);
            }
            _ => break,
        }
    }
    let token = Token::Lit(Lit::Int(value));
    Some(token)
}

/// Perform lexical analysis, converts the input string into
/// a sequence of tokens.
pub fn lex<'a>(input: &'a str) -> Result<Vec<Token>> {
    let mut cursor = Cursor::new(input);
    let mut tokens = Vec::new();
    while !cursor.is_empty() {
        // TODO: Munch comments

        if let Some(_) = munch_whitespace(&mut cursor) {
            continue;
        }

        if let Some(token) = munch_punctuation(&mut cursor) {
            tokens.push(token);
            continue;
        }

        if let Some(token) = munch_keyword(&mut cursor) {
            tokens.push(token);
            continue;
        }

        if let Some(token) = munch_identifier(&mut cursor) {
            tokens.push(token);
            continue;
        }

        if let Some(token) = munch_literal_string(&mut cursor) {
            tokens.push(token);
            continue;
        }

        if let Some(token) = munch_literal_integer(&mut cursor) {
            tokens.push(token);
            continue;
        }

        // TODO: Munch literal float

        // TODO: Munch literal char

        let top = cursor.peek().unwrap();
        bail!("unable to tokenize at index {}: '{}'", cursor.idx, top);
    }
    Ok(tokens)
}
