use core::{ffi, ptr};

use open62541_sys::{
    va_list_, UA_Client_delete, UA_Client_new, UA_LogCategory, UA_LogLevel, UA_Logger,
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
