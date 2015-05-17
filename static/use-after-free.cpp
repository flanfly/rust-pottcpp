#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define SIZE 1024

namespace {
    void logError(const char* msg, int* ints) {
        fprintf(stderr, "%s: %i\n", msg, ints[0]);
    }
}

int main() {
    int abrt = 0;
    int err = 0;
    int* ptr = (int*)malloc (SIZE);
    // ...
    err = 1;
    if (err) {
        abrt = 1;
        free(ptr);
    }
    // ...
    if (abrt) {
        logError("operation aborted before commit", ptr);
    }
}

