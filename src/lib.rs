// Disable several lints. The auto-generated bindings do not conform to them.
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::transmute_int_to_bool)]
#![allow(clippy::type_complexity)]
#![allow(clippy::useless_transmute)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// For some reason, `bindgen` generates different type signatures with variadic arguments. We try to
// export a single type that can be used instead, e.g. in `UA_Logger::log`.
#[cfg(all(unix, target_arch = "x86_64"))]
pub type va_list_ = *mut crate::__va_list_tag;
#[cfg(not(all(unix, target_arch = "x86_64")))]
pub type va_list_ = crate::va_list;

/// Callback type used for [`UA_Logger::log`].
pub type UA_LoggerLogCallback_ = Option<
    unsafe extern "C" fn(
        logContext: *mut ffi::c_void,
        level: crate::UA_LogLevel,
        category: crate::UA_LogCategory,
        msg: *const ffi::c_char,
        // Use unified type from above.
        args: crate::va_list_,
    ),
>;

/// Callback type used for [`UA_Logger::clear`].
pub type UA_LoggerClearCallback_ = Option<unsafe extern "C" fn(logger: *mut UA_Logger)>;
