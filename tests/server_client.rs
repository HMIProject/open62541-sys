use std::{
    ffi::CString,
    slice, str,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use open62541_sys::{
    UA_Array_copy, UA_Array_delete, UA_Client, UA_Client_connect, UA_Client_delete,
    UA_Client_disconnect, UA_Client_new, UA_Client_readValueAttribute, UA_DataType, UA_Server,
    UA_ServerConfig, UA_Server_delete, UA_Server_getConfig, UA_Server_new, UA_Server_run_iterate,
    UA_Server_run_shutdown, UA_Server_run_startup, UA_String, UA_String_clear, UA_String_delete,
    UA_String_fromChars, UA_String_new, UA_Variant, UA_Variant_delete, UA_Variant_new, UA_print,
    UA_NODEID_NUMERIC, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME, UA_STATUSCODE_GOOD,
    UA_TYPES, UA_TYPES_STRING, UA_TYPES_VARIANT,
};

#[test]
fn create_and_destroy_client() {
    // This does not actually connect to anything.
    let client = unsafe { UA_Client_new() };
    unsafe { UA_Client_delete(client) };
}

#[test]
fn open_server_and_connect() {
    // Run server, then connect to it and read value.
    run_server(|discovery_url| {
        run_client(discovery_url, |client| {
            let value = unsafe { UA_Variant_new() };

            let res = unsafe {
                UA_Client_readValueAttribute(
                    client,
                    UA_NODEID_NUMERIC(0, UA_NS0ID_SERVER_SERVERSTATUS_BUILDINFO_PRODUCTNAME),
                    value,
                )
            };
            assert_eq!(res, UA_STATUSCODE_GOOD, "read value");
            assert!(print_variant(value).contains("open62541 OPC UA Server"));

            unsafe { UA_Variant_delete(value) };
        });
    });
}

/// Runs code for active server.
fn run_server(f: impl FnOnce(&str)) {
    // Newtype implements `Send`.
    struct Server(*mut UA_Server);
    unsafe impl Send for Server {}

    // Initialize new server listening on random port.
    let server = Server(unsafe { UA_Server_new() });
    set_server_url(
        unsafe { UA_Server_getConfig(server.0) },
        "opc.tcp://127.0.0.1:0",
    );

    // Variables for communication across thread boundary.
    let running = Arc::new(AtomicBool::new(true));
    let (discovery_url_tx, discovery_url_rx) = std::sync::mpsc::channel();

    // Run server in background thread, iterating event loop.
    let background_thread = thread::spawn({
        let running = Arc::clone(&running);
        move || {
            // Force move of entire `server` (not `server.0`).
            let server = server;
            let server = server.0;

            let res = unsafe { UA_Server_run_startup(server) };
            assert_eq!(res, UA_STATUSCODE_GOOD, "start up server");
            discovery_url_tx
                .send(discovery_url(server).expect("discovery URL"))
                .expect("client still running");

            while running.load(Ordering::Relaxed) {
                unsafe { UA_Server_run_iterate(server, true) };
            }

            let res = unsafe { UA_Server_run_shutdown(server) };
            assert_eq!(res, UA_STATUSCODE_GOOD, "shut down server");

            unsafe { UA_Server_delete(server) };
        }
    });

    // In main thread, wait for discovery URL (after start-up).
    let discovery_url = discovery_url_rx
        .recv_timeout(Duration::from_secs(1))
        .expect("server still running");

    // Run user-provided code.
    f(&discovery_url);

    // Shut down server.
    running.store(false, Ordering::Relaxed);
    background_thread.join().expect("no panic in server");
}

/// Runs code for active client.
fn run_client(url: &str, f: impl FnOnce(*mut UA_Client)) {
    let client = unsafe { UA_Client_new() };

    // Connect to given URL.
    let url = CString::new(url).expect("valid C string");
    let res = unsafe { UA_Client_connect(client, url.as_ptr()) };
    assert_eq!(res, UA_STATUSCODE_GOOD, "connect client");

    f(client);

    // Disconnect client.
    let res = unsafe { UA_Client_disconnect(client) };
    assert_eq!(res, UA_STATUSCODE_GOOD, "disconnect client");

    // Clean up client.
    unsafe { UA_Client_delete(client) };
}

/// Sets server URL.
fn set_server_url(config: *mut UA_ServerConfig, url: &str) {
    let config = unsafe { config.as_mut() }.expect("server config");

    let url = CString::new(url).expect("valid C string");
    // Create stack-allocated `UA_String` from incoming data.
    let mut url = unsafe { UA_String_fromChars(url.as_ptr()) };

    // Remove existing server URLs set by `UA_ServerConfig_setDefault()` in `UA_Server_new()`.
    if config.serverUrlsSize > 0 {
        unsafe {
            UA_Array_delete(
                config.serverUrls.cast(),
                config.serverUrlsSize,
                data_type(UA_TYPES_STRING),
            );
        }
    }

    // Use array helpers to re-allocate memory as expected by open62541. In particular, this must be
    // done to copy data from the stack to the heap.
    let res = unsafe {
        UA_Array_copy(
            (&raw const url).cast(),
            1,
            (&raw mut config.serverUrls).cast(),
            data_type(UA_TYPES_STRING),
        )
    };
    assert_eq!(res, UA_STATUSCODE_GOOD, "copy server URL");
    config.serverUrlsSize = 1;

    // Release heap portion of stack-allocated temporary.
    unsafe { UA_String_clear(&raw mut url) };
}

/// Extracts discovery URL from server.
fn discovery_url(server: *mut UA_Server) -> Option<String> {
    let config = unsafe { UA_Server_getConfig(server).as_mut() }.expect("server config");

    (config.applicationDescription.discoveryUrlsSize != 0)
        .then(|| unsafe { string_as_str(config.applicationDescription.discoveryUrls) }.to_owned())
}

/// Prints contents of variant.
fn print_variant(value: *const UA_Variant) -> String {
    let output = unsafe { UA_String_new() };

    let res = unsafe { UA_print(value.cast(), data_type(UA_TYPES_VARIANT), output) };
    assert_eq!(res, UA_STATUSCODE_GOOD, "print value");

    // Copy out value before releasing temporary local data.
    let value = unsafe { string_as_str(output) }.to_owned();
    unsafe { UA_String_delete(output) };

    value
}

/// Gets string slice from [`UA_String`].
///
/// # Safety
///
/// The slice points to the memory of the given value. It must not be accessed after the value is no
/// longer available.
unsafe fn string_as_str<'a>(value: *const UA_String) -> &'a str {
    let url = unsafe { value.as_ref() }.expect("valid string");

    str::from_utf8(unsafe { slice::from_raw_parts(url.data, url.length) }).expect("valid UTF-8")
}

/// Returns indexed data type.
///
/// This must be called with one of the `UA_TYPES_...` indices.
fn data_type(index: u32) -> *const UA_DataType {
    let index = usize::try_from(index).expect("valid data type");

    #[expect(clippy::indexing_slicing, reason = "valid index")]
    unsafe {
        &raw const UA_TYPES[index]
    }
}
