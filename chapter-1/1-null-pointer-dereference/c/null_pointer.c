#include <stdio.h>

int main() {
	int *ptr = NULL;
	*ptr = 5;
	return 0;
}

// gcc null_pointer.c -o null_pointer && ./null_pointer
// Output: Segmentation fault (core dumped)
