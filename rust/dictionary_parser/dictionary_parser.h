#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

extern "C" {

uintptr_t dictionary_find_items(const void *content,
                                const void *search_key,
                                bool is_text_search,
                                bool is_wildcard_search,
                                void **keys_buffer,
                                void **values_buffer,
                                uintptr_t buffer_length);

uintptr_t tabledictionaryengine_collect_word(const void *engine,
                                             const void *search_key,
                                             void **keys_buffer,
                                             void **values_buffer,
                                             uintptr_t buffer_length);

uintptr_t tabledictionaryengine_collect_word_for_wildcard(const void *engine,
                                                          const void *search_key,
                                                          void **keys_buffer,
                                                          void **values_buffer,
                                                          uintptr_t buffer_length);

uintptr_t tabledictionaryengine_collect_word_from_converted_string_for_wildcard(const void *engine,
                                                                                const void *search_key,
                                                                                void **keys_buffer,
                                                                                void **values_buffer,
                                                                                uintptr_t buffer_length);

void tabledictionaryengine_free(void *engine);

void *tabledictionaryengine_load(const void *path, bool sort);

} // extern "C"
