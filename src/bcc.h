#ifndef BCC_H
#define BCC_H

#include <stdalign.h>
#include <stdbool.h>
#include <stddef.h>
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
 * alloc.c
 */

void *bcc_alloc(size_t size);

void bcc_free(void);

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

struct TokenStream {
	struct Token *buf;
	size_t cap;
	size_t len;
};

struct TokenStream token_stream_new(void);

void token_stream_push(struct TokenStream *s, struct Token t);

/**
 * lex.c
 */

bool lex(struct Str input, struct TokenStream *output);

/**
 * ast.c
 */

enum CTypeType {
	// Simple type
	TYPE_VOID,
	TYPE_CHAR,
	TYPE_INT,
	TYPE_DOUBLE,
	// Compound types
	TYPE_PTR,
	TYPE_STRUCT,
};

enum AstNodeType {
	AST_PROGRAM,
	AST_FN_DEFN,
	AST_PARAM_LIST,
	AST_FN_BODY,
	AST_DECL_STMT,
	AST_EXPR_STMT,
	AST_TYPED_IDENT,
	AST_TYPE_PREFIX
};

struct AstNode {
	enum AstNodeType type;
	union {
		// A C program
		struct {
			struct AstNode *items;
			size_t item_count;
		} program;

		// A function definition
		struct {
			struct AstNode *typed_ident;
			struct AstNode *param_list;
			struct AstNode *fn_body;
		} fn_defn;

		// A parameter list (if empty, then "void")
		struct {
			struct TypedIdents *idents;
			size_t len;
		} param_list;

		// A function body
		struct {
			struct AstNode *statements;
			size_t statement_count;
		} fn_body;

		// A declaration statement
		struct {
			struct AstNode *typed_ident;
			bool has_init_expr;
			struct AstNode *expr;
		} decl_statement;

		// An expression statement
		struct {
			struct AstNode *expr;
		} expr_statement;

		// A typed identifier
		struct {
			struct String ident;
			struct AstNode *type_prefix;
		} typed_ident;

		// A type prefix
		struct {
			enum {
				TYPE_PREFIX_INT,
				TYPE_PREFIX_CHAR,
				TYPE_PREFIX_STRUCT,
			} type;
			union {
				struct String struct_ident;
			} meta;
		} type_prefix;

		// An identifier expression
		struct {
			struct String ident;
		} ident_expr;

		// A parenthesized expression
		struct {
			struct AstNode *expr;
		} paren_expr;

		// A binary operator expression
		struct {
			struct AstNode *expr_left;
			struct String bin_op;
			struct AstNode *expr_right;
		} bin_op_expr;

		// An assignment expression
		struct {
			struct AstNode *expr_left;
			struct AstNode *expr_right;
		} assn_expr;
	} props;
};

void ast_debug(struct AstNode *node, int depth);

/**
 * parse.c
 */

struct AstNode *parse(struct TokenStream *input);

#endif
