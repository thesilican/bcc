#ifndef BCC_H
#define BCC_H

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * utils.c
 */

/**
 * Immediately abort execution with a provided message.
 */
void panic(char *msg);

/*
 * string.c
 */

/**
 * A heap allocated string annotated with its length.
 * Ascii only for now because I'm too lazy to properly handle unicode.
 */
struct String {
	char *buf;
	size_t cap;
	size_t len;
};

/**
 * A borrowed string slice.
 */
struct Str {
	char *ptr;
	size_t len;
};

struct String string_new(void);

struct String string_from_str(struct Str s);

struct String string_from_cstr(char *cstr);

void string_push(struct String *s, struct Str val);

struct Str string_slice(struct String *s, size_t start, size_t end);

struct Str string_as_ref(struct String *s);

void string_drop(struct String *s);

struct Str str_from_cstr(char *str);

struct Str str_slice(struct Str s, size_t start, size_t end);

bool str_eq(struct Str a, struct Str b);

void str_print(struct Str s);

void str_println(struct Str s);

/*
 * io.c
 */

struct String file_read_to_string(FILE *f);

struct String file_read_by_filename(char *filename);

/*
 * token.c
 */

enum TokenType {
	/** Identifier */
	TOKEN_IDENT,
	/** Keyword */
	TOKEN_KW,
	/** Literal */
	TOKEN_LIT,
	/** Punctuation */
	TOKEN_PUNCT
};

enum LitType {
	/** Integer */
	LIT_INT,
	/** Character */
	LIT_CHAR,
	/** Real number */
	LIT_REAL,
	/** String */
	LIT_STR
};

enum PunctType {
	/** ( */
	PUNCT_LPAREN,
	/** ) */
	PUNCT_RPAREN,
	/** [ */
	PUNCT_LBRACKET,
	/** ] */
	PUNCT_RBRACKET,
	/** { */
	PUNCT_LBRACE,
	/** } */
	PUNCT_RBRACE,
	/** ; */
	PUNCT_SEMI,
	/** , */
	PUNCT_COMMA,
	/** . */
	PUNCT_DOT,
	/** : */
	PUNCT_COLON,
	/** * */
	PUNCT_STAR,
};

struct Token {
	/** The type of the token */
	enum TokenType type;
	/** String span of the token */
	struct String span;
	/** Additional metadata, value on type */
	union {
		enum LitType lit;
		enum PunctType punct;
	} meta;
};

void token_debug(struct Token *t);

void token_drop(struct Token *t);

struct TokenStream {
	struct Token *buf;
	size_t cap;
	size_t len;
};

struct TokenStream token_stream_new(void);

void token_stream_drop(struct TokenStream *s);

void token_stream_push(struct TokenStream *s, struct Token t);

/**
 * lex.c
 */

bool lex(struct Str input, struct TokenStream *output);

#endif
