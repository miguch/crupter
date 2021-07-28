#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t status;
  char *name;
  char *result;
  uint64_t current;
  uint64_t bytes;
} StatusInfo;

uint32_t add_hash_file(const char *path_raw);

void clean_hash_files(void);

void free_rust_cstr(char *ptr);

void free_status_info(StatusInfo item);

char *get_hash_file(uint32_t index);

uint32_t get_hash_files_num(void);

StatusInfo get_hash_progress(uint32_t index);

uint32_t hash_running_count(void);

void print_info(StatusInfo info);

void remove_hash_file(uint32_t index);

void run_hash_session(void);
