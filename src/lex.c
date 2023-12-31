#include "bcc.h"

static size_t munch_whitespace(struct Str input, size_t idx) {
	size_t count = 0;
	while (idx < input.len && input.ptr[idx]) {
		char c = input.ptr[idx];
		if (!(c == ' ' || c == '\n' || c == '\t')) {
			break;
		}
		idx++;
		count++;
	}
	return count;
}

static size_t munch_keyword(struct Str input, size_t idx,
							struct Token *output) {
	struct Str KEYWORDS[] = {
		str_from_cstr("break"),	  str_from_cstr("case"),
		str_from_cstr("char"),	  str_from_cstr("continue"),
		str_from_cstr("default"), str_from_cstr("do"),
		str_from_cstr("double"),  str_from_cstr("else"),
		str_from_cstr("enum"),	  str_from_cstr("float"),
		str_from_cstr("for"),	  str_from_cstr("goto"),
		str_from_cstr("if"),	  str_from_cstr("int"),
		str_from_cstr("long"),	  str_from_cstr("return"),
		str_from_cstr("short"),	  str_from_cstr("signed"),
		str_from_cstr("sizeof"),  str_from_cstr("static"),
		str_from_cstr("struct"),  str_from_cstr("switch"),
		str_from_cstr("union"),	  str_from_cstr("unsigned"),
		str_from_cstr("void"),	  str_from_cstr("while")};
	size_t KEYWORDS_LEN = sizeof(KEYWORDS) / sizeof(*KEYWORDS);

	for (size_t i = 0; i < KEYWORDS_LEN; i++) {
		size_t len = KEYWORDS[i].len;
		if (idx + len > input.len) {
			continue;
		}
		struct Str slice = str_slice(input, idx, idx + len);
		if (str_eq(KEYWORDS[i], slice)) {
			output->type = TOKEN_KW;
			output->span = string_from_str(slice);
			return len;
		}
	}
	return 0;
}

static size_t munch_ident(struct Str input, size_t idx, struct Token *output) {
	char c = input.ptr[idx];
	if (!((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_')) {
		return 0;
	}
	size_t count = 1;
	while (idx + count < input.len) {
		c = input.ptr[idx + count];
		if (!((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
			  (c >= '0' && c <= '9') || c == '_')) {
			break;
		}
		count++;
	}
	output->type = TOKEN_IDENT;
	output->span = string_from_str(str_slice(input, idx, idx + count));
	return count;
}

struct PunctPair {
	enum PunctType punct;
	struct Str str;
};

static size_t munch_punct(struct Str input, size_t idx, struct Token *output) {
	struct PunctPair PUNCTS[] = {{PUNCT_LPAREN, str_from_cstr("(")},
								 {PUNCT_RPAREN, str_from_cstr(")")},
								 {PUNCT_LBRACKET, str_from_cstr("[")},
								 {PUNCT_RBRACKET, str_from_cstr("]")},
								 {PUNCT_LBRACE, str_from_cstr("{")},
								 {PUNCT_RBRACE, str_from_cstr("}")},
								 {PUNCT_SEMI, str_from_cstr(";")},
								 {PUNCT_COMMA, str_from_cstr(",")},
								 {PUNCT_DOT, str_from_cstr(".")},
								 {PUNCT_COLON, str_from_cstr(":")},
								 {PUNCT_STAR, str_from_cstr("*")}};
	size_t PUNCTS_LEN = sizeof(PUNCTS) / sizeof(*PUNCTS);

	for (size_t i = 0; i < PUNCTS_LEN; i++) {
		struct PunctPair pair = PUNCTS[i];
		enum PunctType punct = pair.punct;
		struct Str str = pair.str;
		size_t len = str.len;
		if (idx + len > input.len) {
			continue;
		}
		struct Str slice = str_slice(input, idx, idx + len);
		if (str_eq(str, slice)) {
			output->type = TOKEN_PUNCT;
			output->span = string_from_str(slice);
			output->meta.punct = punct;
			return len;
		}
	}
	return 0;
}

static size_t munch_lit(struct Str input, size_t idx, struct Token *output) {
	char c = input.ptr[idx];
	if (c >= '0' && c <= '9') {
		// TODO: Handle floating point numbers
		size_t count = 1;
		while (idx + count > input.len) {
			c = input.ptr[idx + count];
			if (!(c >= '0' && c <= '9')) {
				break;
			}
			count++;
		}
		output->type = TOKEN_LIT;
		output->span = string_from_str(str_slice(input, idx, idx + count));
		output->meta.lit = LIT_INT;
		return count;
	} else if (c == '\'') {
		// TODO: Handle char parsing
	} else if (c == '\"') {
		// TODO: Handle string parsing
	}
	return 0;
}

bool lex(struct Str input, struct TokenStream *output) {
	size_t idx = 0;
	while (idx < input.len) {
		size_t count;
		struct Token t;
		// Whitespace
		count = munch_whitespace(input, idx);
		if (count > 0) {
			idx += count;
			continue;
		}

		// TODO: Strip comments

		// Keyword
		count = munch_keyword(input, idx, &t);
		if (count > 0) {
			idx += count;
			token_stream_push(output, t);
			continue;
		}

		// Identifier
		count = munch_ident(input, idx, &t);
		if (count > 0) {
			idx += count;
			token_stream_push(output, t);
			continue;
		}

		// Punct
		count = munch_punct(input, idx, &t);
		if (count > 0) {
			idx += count;
			token_stream_push(output, t);
			continue;
		}

		// Lit
		count = munch_lit(input, idx, &t);
		if (count > 0) {
			idx += count;
			token_stream_push(output, t);
			continue;
		}

		printf("Error lexing at index %lu\n", idx);
		return false;
	}
	return true;
}
