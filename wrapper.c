#include "wrapper.h"

// Wrapper for `vsnprintf()` with normalized behavior across different platforms
// such as Microsoft Windows.
#if defined(_MSC_VER) && _MSC_VER < 1900
int RS_vsnprintf(
    char *buffer,
    size_t count,
    const char *format,
    va_list argptr)
{
  // Microsoft does not (always) define a standard-conforming `vsnprintf()`. But
  // it does define a variant with slightly different behavior. We normalize the
  // differences as best we can.
  int result = -1;
  if (count)
    result = _vsnprintf_s(buffer, count, _TRUNCATE, format, argptr);
  if (result < 0)
    result = _vscprintf(format, argptr);
  return result;
}
#else
int RS_vsnprintf(
    char *buffer,
    size_t count,
    const char *format,
    va_list argptr)
{
  // Forward to existing standards-compliant function. It may have be defined as
  // a macro, so we need a wrapper function for bindgen to pick it up anyway.
  return vsnprintf(buffer, count, format, argptr);
}
#endif

// bindgen does not support non-trivial `#define` used for pointer constant. Use
// statically defined constant as workaround for now.
//
// See https://github.com/rust-lang/rust-bindgen/issues/2426
const void *const RS_EMPTY_ARRAY_SENTINEL = UA_EMPTY_ARRAY_SENTINEL;
