#include "bcc.h"

struct String file_read_to_string(FILE *f) {
	size_t count;
	char buf[256];
	struct String s = string_new();
	while (true) {
		count = fread(buf, 1, 256, f);
		if (count == 0) {
			break;
		}
		struct Str str = {buf, count};
		string_push(&s, str);
	}
	return s;
}

struct String file_read_by_filename(char *filename) {
	struct String output;
	if (strcmp(filename, "-") == 0) {
		output = file_read_to_string(stdin);
	} else {
		FILE *f = fopen(filename, "r");
		if (f == NULL) {
			panic("error opening file");
		}
		output = file_read_to_string(f);
		fclose(f);
	}
	return output;
}
