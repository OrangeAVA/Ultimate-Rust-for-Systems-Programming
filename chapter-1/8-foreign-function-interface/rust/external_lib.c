#include "external_lib.h"
#include <stdio.h>

void process_data(unsigned char *data, size_t length) {
    for (size_t i = 0; i < length; i++) {
        printf("%u ", data[i]);
    }
    printf("\n");
}
