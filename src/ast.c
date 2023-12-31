#include "bcc.h"

void ast_debug(struct AstNode *node, int depth) {
	for (int i = 0; i < depth; i++) {
		printf("\t");
	}

	switch (node->type) {
	case AST_PROGRAM:
		break;

	default:
		panic("unknown ast type");
		break;
	}
	printf("\n");
}
