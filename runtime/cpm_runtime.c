#include "cpm_runtime.h"
#include <stdio.h>

#ifdef _WIN32
#include <windows.h>
#endif

void cpm_init(void) {
#ifdef _WIN32
    SetConsoleOutputCP(CP_UTF8);
    SetConsoleCP(CP_UTF8);
#endif
}

void cpm_print_int(long long value) {
    printf("%lld", value);
}

void cpm_print_str(const char* value) {
    printf("%s", value);
}
