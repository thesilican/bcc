#include "bcc.h"

void token_debug(struct Token *t) {
	switch (t->type) {
	case TOKEN_IDENT:
		printf("TOKEN_IDENT ");
		break;
	case TOKEN_KW:
		printf("TOKEN_KW ");
		break;
	case TOKEN_LIT:
		printf("TOKEN_LIT ");
		break;
	case TOKEN_PUNCT:
		printf("TOKEN_PUNCT ");
		break;
	default:
		panic("unreachable");
		break;
	}
	str_print(string_as_ref(&t->span));
	printf("\n");
}

void token_drop(struct Token *t) {
	string_drop(&t->span);
}

struct TokenStream token_stream_new(void) {
	int cap = 1;
	struct Token *buf = malloc(sizeof(*buf) * cap);
	if (buf == NULL) {
		panic("malloc error");
	}

	struct TokenStream output = {.buf = buf, .cap = cap, .len = 0};
	return output;
}

static void token_stream_realloc(struct TokenStream *s, size_t new_cap) {
	struct Token *new_buf = realloc(s->buf, sizeof(*new_buf) * new_cap);
	if (new_buf == NULL) {
		panic("realloc error");
	}
	s->buf = new_buf;
	s->cap = new_cap;
}

void token_stream_drop(struct TokenStream *s) {
	for (size_t i = 0; i < s->len; i++) {
		token_drop(&s->buf[i]);
	}
	free(s->buf);
	s->buf = NULL;
	s->cap = 0;
	s->len = 0;
}

void token_stream_push(struct TokenStream *s, struct Token t) {
	if (s->len == s->cap) {
		token_stream_realloc(s, s->cap * 2);
	}
	s->buf[s->len] = t;
	s->len++;
}
