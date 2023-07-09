use std::ffi;
use std::mem::MaybeUninit;

use open62541_sys;

fn main() {
    unsafe {
        let client = open62541_sys::UA_Client_new();
        open62541_sys::UA_ClientConfig_setDefault(open62541_sys::UA_Client_getConfig(client));
        let retval = open62541_sys::UA_Client_connect(
            client,
            ffi::CString::new("opc.tcp://opcua.demo-this.com:51210")
                .unwrap()
                .as_bytes_with_nul() as *const _ as *const i8,
        );
        if retval != open62541_sys::UA_STATUSCODE_GOOD {
            open62541_sys::UA_Client_delete(client);
            return;
        }

        let mut value = MaybeUninit::uninit();
        open62541_sys::UA_Variant_init(value.as_mut_ptr());
        let mut value = value.assume_init();

        let node_id = open62541_sys::UA_NODEID_NUMERIC(
            0,
            open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
        );
        let retval = open62541_sys::UA_Client_readValueAttribute(client, node_id, &mut value);

        if retval == open62541_sys::UA_STATUSCODE_GOOD
            && open62541_sys::UA_Variant_hasScalarType(
                &value,
                &open62541_sys::UA_TYPES[open62541_sys::UA_TYPES_DATETIME as usize],
            )
        {
            let raw_date = value.data as *const open62541_sys::UA_DateTime;
            let dts = open62541_sys::UA_DateTime_toStruct(*raw_date);
            println!("{dts:?}");
        }

        open62541_sys::UA_Variant_clear(&mut value);
        open62541_sys::UA_Client_delete(client);
    }
}
