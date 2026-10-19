#ifndef CPM_RUNTIME_H
#define CPM_RUNTIME_H

void cpm_init(void);
void cpm_print_int(long long value);
void cpm_print_str(const char* value);
long long cpm_time(void);
long long cpm_random(void);
long long cpm_read_int(void);
const char* cpm_read_str(void);

#endif
