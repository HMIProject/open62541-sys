use std::ffi;
use std::mem::MaybeUninit;

use open62541_sys;

fn main() {
    unsafe {
        let client = open62541_sys::UA_Client_new();
        open62541_sys::UA_ClientConfig_setDefault(open62541_sys::UA_Client_getConfig(client));
        let retval = open62541_sys::UA_Client_connect(
            client,
            ffi::CString::new("opc.tcp://localhost:4840")
                .unwrap()
                .as_bytes_with_nul() as *const _ as *const i8,
        );
        if retval != open62541_sys::UA_STATUSCODE_GOOD {
            open62541_sys::UA_Client_delete(client);
            return;
        }

        let mut value: open62541_sys::UA_Variant = MaybeUninit::zeroed().assume_init();

        let node_id = open62541_sys::UA_NodeId {
            namespaceIndex: 0,
            identifierType: open62541_sys::UA_NodeIdType_UA_NODEIDTYPE_NUMERIC,
            identifier: open62541_sys::UA_NodeId__bindgen_ty_1 {
                numeric: open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
            },
        };
        let _retval = open62541_sys::__UA_Client_readAttribute(
            client,
            &node_id,
            open62541_sys::UA_AttributeId_UA_ATTRIBUTEID_VALUE,
            &mut value as *mut _ as *mut ffi::c_void,
            &open62541_sys::UA_TYPES[open62541_sys::UA_TYPES_VARIANT as usize],
        );

        if retval == open62541_sys::UA_STATUSCODE_GOOD {
            println!("{value:?}");
        }

        open62541_sys::UA_clear(
            &mut value as *mut _ as *mut ffi::c_void,
            &open62541_sys::UA_TYPES[open62541_sys::UA_TYPES_VARIANT as usize],
        );
        open62541_sys::UA_Client_delete(client);
    }
}
