#include "bcc.h"

static void *arena = NULL;
static size_t offset = 0;
#define MAX_HEAP_SIZE 65536
#define ALIGN 8

void *bcc_alloc(size_t size) {
	if (arena == NULL) {
		arena = malloc(MAX_HEAP_SIZE);
		if (arena == NULL) {
			panic("malloc error");
		}
	}
	if (offset + size > MAX_HEAP_SIZE) {
		panic("heap out of memory");
	}
	void *ptr = (uint8_t *)arena + offset;
	offset += size;
	// Pad so that offset is a multiple of ALIGN
	size_t padding = offset - (offset / ALIGN) * ALIGN;
	offset += padding;
	return ptr;
}

void bcc_free(void) {
	free(arena);
	arena = NULL;
}
