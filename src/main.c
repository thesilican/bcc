#include "bcc.h"

static void test_string(char *filename) {
	// Read file
	struct String contents = file_read_by_filename(filename);

	// Print that file
	str_print(string_as_ref(&contents));

	// Free used memory
	bcc_free();
}

static void test_lex(char *filename) {
	// Read file
	struct String contents = file_read_by_filename(filename);

	// Perform lexing
	struct TokenStream tokens = token_stream_new();
	lex(string_as_ref(&contents), &tokens);

	for (size_t i = 0; i < tokens.len; i++) {
		struct Token *t = &tokens.buf[i];
		token_debug(t);
	}

	// Free used memory
	bcc_free();
}

static void test_parse(char *filename) {
	// Read file
	struct String contents = file_read_by_filename(filename);

	// Lex
	struct TokenStream tokens = token_stream_new();
	lex(string_as_ref(&contents), &tokens);

	// Parse
	struct AstNode *root = parse(&tokens);

	ast_debug(root, 0);

	// Free used memory
	bcc_free();
}

int main(int argc, char **argv) {
	if (argc < 2) {
		printf("Expected at least 1 argument\n");
		return 1;
	}

	if (strcmp(argv[1], "test") == 0) {
		if (argc < 3) {
			printf("Usage: bcc test <subcommand> [args...]\n");
			return 1;
		}

		if (strcmp(argv[2], "lex") == 0) {
			if (argc < 4) {
				printf("Usage: bcc test lex <filename>\n");
				return 1;
			}
			test_lex(argv[3]);
			return 0;
		}
		if (strcmp(argv[2], "string") == 0) {
			if (argc < 4) {
				printf("Usage: bcc test string <filename>\n");
				return 1;
			}
			test_string(argv[3]);
			return 0;
		}
		if (strcmp(argv[2], "parse") == 0) {
			if (argc < 4) {
				printf("Usage: bcc test parse <filename>\n");
				return 1;
			}
			test_parse(argv[3]);
			return 0;
		}
	}

	printf("Unknown usage\n");
	return 1;
}
