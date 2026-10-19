#include "cpm_runtime.h"
#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <stdint.h>

#ifdef _WIN32
#include <windows.h>
#include <bcrypt.h>
#elif defined(__linux__)
#include <sys/random.h>
#include <errno.h>
#elif defined(__APPLE__)
#else
#error "Unsupported platform for cpm_random"
#endif

void cpm_init(void) {
#ifdef _WIN32
    SetConsoleOutputCP(CP_UTF8);
    SetConsoleCP(CP_UTF8);
#endif
}

long long cpm_random(void) {
    uint32_t value;

#ifdef _WIN32
    if (BCryptGenRandom(NULL, (PUCHAR)&value, sizeof(value),
                       BCRYPT_USE_SYSTEM_PREFERRED_RNG) != 0) {
        fprintf(stderr, "cpm_random: Windows RNG failed\n");
        exit(EXIT_FAILURE);
    }
#elif defined(__linux__)
    unsigned char *p = (unsigned char *)&value;
    size_t done = 0;

    while (done < sizeof(value)) {
        ssize_t n = getrandom(p + done, sizeof(value) - done, 0);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) {
            fprintf(stderr, "cpm_random: Linux RNG failed\n");
            exit(EXIT_FAILURE);
        }
        done += (size_t)n;
    }
#elif defined(__APPLE__)
    arc4random_buf(&value, sizeof(value));
#endif

    return (long long)(value & 0x7FFFFFFFu);
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
