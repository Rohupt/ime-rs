#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <minwindef.h>

enum class CandidateMode {
  None,
  Original,
  Incremental,
  WithNextComposition,
};

enum class KeystrokeCategory {
  None,
  Composing,
  Candidate,
  InvokeCompositionEditSession,
};

enum class KeystrokeFunction {
  None,
  Input,
  Cancel,
  FinalizeTextstore,
  FinalizeTextstoreAndInput,
  FinalizeCandidatelist,
  FinalizeCandidatelistAndInput,
  Convert,
  ConvertWildcard,
  SelectByNumber,
  Backspace,
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,
  MovePageUp,
  MovePageDown,
  MovePageTop,
  MovePageBottom,
  DoubleSingleByte,
  Punctuation,
};

extern "C" {

void *compositionprocessorengine_new(ITfThreadMgr* thread_mgr, uint32_t tf_client_id);

void compositionprocessorengine_free(void *engine);

void compositionprocessorengine_test_virtual_key(void *engine,
                                                 uint16_t code,
                                                 uint16_t ch,
                                                 bool composing,
                                                 CandidateMode candidate_mode,
                                                 bool *key_eaten,
                                                 KeystrokeCategory *keystroke_category,
                                                 KeystrokeFunction *keystroke_function);

bool compositionprocessorengine_add_virtual_key(void *engine, uint16_t wch);

void compositionprocessorengine_pop_virtual_key(void *engine);

void compositionprocessorengine_purge_virtual_key(void *engine);

bool compositionprocessorengine_has_virtual_key(void *engine);

void *compositionprocessorengine_get_reading_string(void *engine);

bool compositionprocessorengine_keystroke_buffer_includes_wildcard(void *engine);

void compositionprocessorengine_setup_dictionary_file(void *engine,
                                                      HINSTANCE dll_instance_handle,
                                                      void *dictionary_file_name);

const void *compositionprocessorengine_get_table_dictionary_engine(const void *engine);

void compositionprocessorengine_modifiers_update(void *engine, WPARAM w, LPARAM l);

bool compositionprocessorengine_modifiers_is_shift_key_down_only(void *engine);

bool compositionprocessorengine_modifiers_is_control_key_down_only(void *engine);

bool compositionprocessorengine_modifiers_is_alt_key_down_only(void *engine);

bool compositionprocessorengine_punctuations_has_alternative_punctuation(void *engine,
                                                                         uint16_t wch);

uint16_t compositionprocessorengine_punctuations_get_alternative_punctuation_counted(void *engine,
                                                                                     uint16_t wch);

HRESULT compositionprocessorengine_preserved_keys_init(void *engine,
                                                       ITfThreadMgr* thread_mgr,
                                                       uint32_t client_id);

HRESULT compositionprocessorengine_on_preserved_key(void *engine,
                                                    const GUID *guid,
                                                    bool *out_is_eaten,
                                                    ITfThreadMgr* thread_mgr,
                                                    uint32_t client_id);

void compositionprocessorengine_compartmentwrapper_conversion_mode_compartment_updated(void *engine,
                                                                                       ITfThreadMgr* thread_mgr);

void compositionprocessorengine_compartmentwrapper_private_compartments_updated(void *engine,
                                                                                ITfThreadMgr* thread_mgr);

const void *compositionprocessorengine_compartmentwrapper_raw_ptr(void *engine);

HRESULT compartment_callback(const void *wrapper, const GUID *guid);

} // extern "C"
