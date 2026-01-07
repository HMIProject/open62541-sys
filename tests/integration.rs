use core::{ffi, ptr};

use open62541_sys::{
    UA_LogCategory, UA_LogLevel, UA_Logger, UA_LoggerClearCallback_, UA_LoggerLogCallback_,
    va_list_,
};

// Check validity of explicitly defined type aliases.
#[test]
fn type_aliases() {
    const unsafe extern "C" fn log_c(
        _log_context: *mut ffi::c_void,
        _level: UA_LogLevel,
        _category: UA_LogCategory,
        _msg: *const ffi::c_char,
        _args: va_list_,
    ) {
        // Nothing here.
    }

    const unsafe extern "C" fn clear_c(_logger: *mut UA_Logger) {
        // Nothing here.
    }

    let log: UA_LoggerLogCallback_ = Some(log_c);
    let clear: UA_LoggerClearCallback_ = Some(clear_c);
    let _logger = UA_Logger {
        log,
        context: ptr::null_mut(),
        clear,
    };
}

// Make sure that our custom exports (prefixed with `RS_`) are available under their expected names,
// i.e. without the `RS_` prefix.
#[test]
fn custom_exports() {
    use open62541_sys::UA_EMPTY_ARRAY_SENTINEL;

    let _unused = unsafe { UA_EMPTY_ARRAY_SENTINEL };
}
