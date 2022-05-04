#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void *ruststringrange_new(const uint16_t *buffer, uintptr_t buffer_len);

void *ruststringrange_new_utf8(const uint8_t *buffer, uintptr_t buffer_len);

void ruststringrange_free(void *p);

const uint8_t *ruststringrange_raw(void *p);

uintptr_t ruststringrange_len(const void *p);

int8_t ruststringrange_compare(void *x_raw, void *y_raw);

bool ruststringrange_compare_with_wildcard(void *x_raw, void *y_raw);

void *ruststringrange_clone(const void *p);

void *ruststringrange_concat(const void *p1, const void *p2);

bool ruststringrange_contains(const void *p, uint8_t ch);

void *ruststringrange_cutlast(void *p);

} // extern "C"
