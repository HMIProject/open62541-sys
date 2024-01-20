use core::{ffi, ptr};

use open62541_sys::{
    va_end, va_list_, vsnprintf, UA_Client_delete, UA_Client_new, UA_LogCategory, UA_LogLevel,
    UA_Logger, UA_LoggerClearCallback_, UA_LoggerLogCallback_,
};

#[test]
fn create_and_destroy_client() {
    let client = unsafe { UA_Client_new() };
    unsafe { UA_Client_delete(client) };
}

#[test]
fn variadic_arguments() {
    // Check if `va_list_` type matches.
    unsafe extern "C" fn log_c(
        _log_context: *mut ffi::c_void,
        _level: UA_LogLevel,
        _category: UA_LogCategory,
        _msg: *const ffi::c_char,
        _args: va_list_,
    ) {
    }
    let _logger = UA_Logger {
        log: Some(log_c),
        context: ptr::null_mut(),
        clear: None,
    };
}

#[test]
fn logger_types() {
    // Check validity of type aliases for `UA_Logger` callbacks.
    unsafe extern "C" fn log_c(
        _log_context: *mut ffi::c_void,
        _level: UA_LogLevel,
        _category: UA_LogCategory,
        _msg: *const ffi::c_char,
        _args: va_list_,
    ) {
    }
    unsafe extern "C" fn clear_c(_context: *mut ffi::c_void) {}
    let log: UA_LoggerLogCallback_ = Some(log_c);
    let clear: UA_LoggerClearCallback_ = Some(clear_c);
    let _logger = UA_Logger {
        log,
        context: ptr::null_mut(),
        clear,
    };
}

#[test]
fn has_vsnprintf() {
    // Make sure that `vsnprintf()` is available. On Microsoft Windows we have to provide a wrapper,
    // to support older versions of the C library (before the introduction of the UCRT/Visual Studio
    // 2015/Windows 10).
    let _unused = vsnprintf;
}

#[test]
fn has_va_end() {
    // Make sure that `va_end()` is available.
    let _unused = va_end;
}
