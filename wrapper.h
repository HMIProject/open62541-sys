#include <open62541/server.h>
#include <open62541/server_config_default.h>
#include <open62541/client.h>
#include <open62541/client_config_default.h>
#include <open62541/client_highlevel.h>
#include <open62541/client_highlevel_async.h>
#include <open62541/client_subscriptions.h>
#include <open62541/plugin/log_stdout.h>
#include <open62541/types.h>

// Include with binding of `vsnprintf()` to simplify formatting of log messages.
#include <stdio.h>

// bindgen does not support non-trivial `#define` used for pointer constant. Use
// statically defined constant as workaround for now.
//
// See https://github.com/rust-lang/rust-bindgen/issues/2426
extern const void *const RS_EMPTY_ARRAY_SENTINEL;

// Wrapper for `vsnprintf()` with normalized behavior across different platforms
// such as Microsoft Windows.
int RS_vsnprintf(
    char *buffer,
    size_t count,
    const char *format,
    va_list argptr);
