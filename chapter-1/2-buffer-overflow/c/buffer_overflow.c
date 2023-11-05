#include <stdio.h>
#include <string.h>

int main() {
	char buffer[5];
	strcpy(buffer, "Overflowing Content!");
	printf("%s\n", buffer);
	return 0;
}

// gcc -Wstringop-overflow=0 -fno-stack-protector buffer_overflow.c -o buffer_overflow && ./buffer_overflow 

// Overflowing Content!
// Segmentation fault (core dumped)
