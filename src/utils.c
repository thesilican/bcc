#include "bcc.h"

void panic(char *msg) {
	printf("panic: %s\n", msg);
	exit(1);
}
