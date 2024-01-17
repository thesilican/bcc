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
 * A borrowed string slice. Must have a shorter lifetime than the
 * underlying string that it is pointing to.
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

// See https://en.cppreference.com/w/c/language/punctuators
enum PunctType {
	/** { */
	PUNCT_LBRACE,
	/** } */
	PUNCT_RBRACE,
	/** [ */
	PUNCT_LBRACKET,
	/** ] */
	PUNCT_RBRACKET,
	/** ( */
	PUNCT_LPAREN,
	/** ) */
	PUNCT_RPAREN,
	/** ; */
	PUNCT_SEMI,
	/** : */
	PUNCT_COLON,
	/** ? */
	PUNCT_QUESTION,
	/** . */
	PUNCT_DOT,
	/** -> */
	PUNCT_ARROW,
	/** ~ */
	PUNCT_TILDE,
	/** ! */
	PUNCT_EXCLAM,
	/** + */
	PUNCT_PLUS,
	/** - */
	PUNCT_DASH,
	/** * */
	PUNCT_STAR,
	/** / */
	PUNCT_SLASH,
	/** % */
	PUNCT_PERCENT,
	/** ^ */
	PUNCT_HAT,
	/** & */
	PUNCT_AMP,
	/** | */
	PUNCT_PIPE,
	/** = */
	PUNCT_EQ,
	/** += */
	PUNCT_PLUSEQ,
	/** -= */
	PUNCT_DASHEQ,
	/** *= */
	PUNCT_STAREQ,
	/** /= */
	PUNCT_SLASHEQ,
	/** %= */
	PUNCT_PERCENTEQ,
	/** ^= */
	PUNCT_HATEQ,
	/** &= */
	PUNCT_AMPEQ,
	/** |= */
	PUNCT_PIPEEQ,
	/** == */
	PUNCT_EQ2,
	/** != */
	PUNCT_EXCLAMEQ,
	/** < */
	PUNCT_LT,
	/** > */
	PUNCT_GT,
	/** <= */
	PUNCT_LTEQ,
	/** >= */
	PUNCT_GTEQ,
	/** && */
	PUNCT_AMP2,
	/** || */
	PUNCT_PIPE2,
	/** << */
	PUNCT_LT2,
	/** >> */
	PUNCT_GT2,
	/** <<= */
	PUNCT_LT2EQ,
	/** >>= */
	PUNCT_GT2EQ,
	/** ++ */
	PUNCT_PLUS2,
	/** -- */
	PUNCT_DASH2,
	/** , */
	PUNCT_COMMA
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

/**
 * ast.c
 */

/**
 * parse.c
 */

// bool parse(struct TokenStream input);

#endif
