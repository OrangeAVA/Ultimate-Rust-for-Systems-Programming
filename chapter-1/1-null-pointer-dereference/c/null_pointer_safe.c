#include <stdio.h>
#include <stdlib.h>

int main() {
	int *ptr = malloc(sizeof(int));
	if (ptr == NULL) { // ①
		printf("Error: memory allocation failed");
	} else {
		*ptr = 5;
	}
}

// gcc null_pointer_safe.c -o null_pointer_safe && ./null_pointer_safe
// Output: Nothing
