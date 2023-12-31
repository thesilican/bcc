#include "bcc.h"

static void main_test_lex(char *filename) {
	// Read file
	struct String contents = file_read_by_filename(filename);

	// Perform lexing
	struct TokenStream tokens = token_stream_new();
	lex(string_as_ref(&contents), &tokens);

	for (size_t i = 0; i < tokens.len; i++) {
		struct Token *t = &tokens.buf[i];
		token_debug(t);
	}

	token_stream_drop(&tokens);
	string_drop(&contents);
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
			main_test_lex(argv[3]);
		} else {
			printf("Unknown usage\n");
			return 1;
		}
	} else {
		printf("Unknown usage\n");
		return 1;
	}
}
