#include "bcc.h"

void panic(char *msg) {
	printf("panic: %s\n", msg);
	bcc_free();
	exit(1);
}
