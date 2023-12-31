#include "bcc.h"

struct String string_new(void) {
	size_t cap = 1;
	size_t len = 0;
	char *buf = bcc_alloc(sizeof(*buf) * cap);
	struct String output = {buf, cap, len};
	return output;
}

struct String string_from_str(struct Str s) {
	size_t len = s.len;
	size_t cap = s.len;

	char *buf = bcc_alloc(sizeof(*buf) * cap);
	memcpy(buf, s.ptr, len);

	struct String output = {buf, cap, len};
	return output;
}

struct String string_from_cstr(char *cstr) {
	struct Str s = str_from_cstr(cstr);
	return string_from_str(s);
}

static void string_realloc(struct String *s, size_t new_cap) {
	char *new_buf = bcc_alloc(sizeof(*new_buf) * new_cap);
	memcpy(new_buf, s->buf, s->len);
	s->buf = new_buf;
	s->cap = new_cap;
}

void string_push(struct String *s, struct Str val) {
	size_t new_len = s->len + val.len;
	if (s->cap < new_len) {
		string_realloc(s, new_len);
	}
	memcpy(s->buf + s->len, val.ptr, val.len);
	s->len = new_len;
}

struct Str string_slice(struct String *s, size_t start, size_t end) {
	if (start >= s->len || end > s->len) {
		panic("string slice out of bounds");
	}
	if (start > end) {
		panic("string slice start > end");
	}

	char *ptr = s->buf + start;
	size_t len = end - start;
	struct Str output = {ptr, len};
	return output;
}

struct Str string_as_ref(struct String *s) {
	struct Str output = {s->buf, s->len};
	return output;
}

struct Str str_from_cstr(char *cstr) {
	size_t len = strlen(cstr);
	struct Str output = {cstr, len};
	return output;
}

struct Str str_slice(struct Str s, size_t start, size_t end) {
	if (start > end || start >= s.len || end > s.len) {
		panic("string slice out of bounds");
	}
	struct Str output = {s.ptr + start, end - start};
	return output;
}

bool str_eq(struct Str a, struct Str b) {
	if (a.len != b.len) {
		return false;
	}
	for (size_t i = 0; i < a.len; i++) {
		if (a.ptr[i] != b.ptr[i]) {
			return false;
		}
	}
	return true;
}

static void put_char_escaped(char c) {
	if (c == '\n') {
		printf("\\n");
	} else if (c == '\r') {
		printf("\\r");
	} else if (c == '\t') {
		printf("\\t");
	} else if (c == '\"') {
		printf("\\\"");
	} else if (c == '\'') {
		printf("\\'");
	} else if (c == '\\') {
		printf("\\\\");
	} else if ((c >= '\x00' && c < '\x20') || c == '\x7f') {
		int low = (int)c % 16;
		int high = (int)c / 16;
		printf("\\x%d%d", high, low);
	} else {
		putchar(c);
	}
}

void str_print(struct Str s) {
	putchar('"');
	for (size_t i = 0; i < s.len; i++) {
		put_char_escaped(s.ptr[i]);
	}
	putchar('"');
}

void str_println(struct Str s) {
	str_print(s);
	putchar('\n');
}
