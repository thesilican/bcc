#include "bcc.h"

static bool munch_program(struct TokenStream *tokens, size_t idx,
						  struct AstNode *output) {
	panic("not implemented");
}

struct AstNode *parse(struct TokenStream *tokens) {
	struct AstNode *output = bcc_alloc(sizeof(*output));
	bool success = munch_program(tokens, 0, output);
	if (!success) {
		panic("error parsing: error munching program");
	}
	return output;
}
