#include "wrapper.h"

// bindgen does not support non-trivial `#define` used for pointer constant. Use
// statically defined constant as workaround for now.
//
// See <https://github.com/rust-lang/rust-bindgen/issues/2426>.
const void *const RS_UA_EMPTY_ARRAY_SENTINEL = UA_EMPTY_ARRAY_SENTINEL;
