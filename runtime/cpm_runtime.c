#include "cpm_runtime.h"
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#ifdef _WIN32
#include <windows.h>
#endif

void cpm_init(void) {
#ifdef _WIN32
    SetConsoleOutputCP(CP_UTF8);
    SetConsoleCP(CP_UTF8);
#endif

    srand((unsigned)time(NULL));
}

long long cpm_random(void) {
    return (long long)rand();
}

long long cpm_time(void) {
    return (long long)time(NULL);
}

long long cpm_read_int(void) {
    long long value = 0;
    scanf("%lld", &value);
    return value;
}

const char* cpm_read_str(void) {
    static char buffer[256];
    if (scanf(" %255[^\n]", buffer) != 1) {
        buffer[0] = '\0';
    }
    return buffer;
}

void cpm_print_int(long long value) {
    printf("%lld", value);
}

void cpm_print_str(const char* value) {
    printf("%s", value);
}
