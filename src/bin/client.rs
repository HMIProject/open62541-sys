fn main() {
    let mut client = ll::Client::new();

    client
        .connect("opc.tcp://opcua.demo-this.com:51210")
        .unwrap();

    let value = client
        .read_value_attribute(&ll::NodeId::numeric(0, 2258))
        .unwrap();

    let value = value.into_date_time();

    println!("{value:?}");

    client.disconnect().unwrap();
}

fn _main_2() {
    use std::ffi;
    use std::mem::MaybeUninit;

    unsafe {
        let client = open62541_sys::UA_Client_new();
        open62541_sys::UA_ClientConfig_setDefault(open62541_sys::UA_Client_getConfig(client));
        let retval = open62541_sys::UA_Client_connect(
            client,
            ffi::CString::new("opc.tcp://opcua.demo-this.com:51210")
                .unwrap()
                .as_bytes_with_nul() as *const _ as *const ffi::c_char,
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

pub mod ll {
    use std::ffi;
    use std::mem::MaybeUninit;

    use open62541_sys::{
        UA_Client, UA_ClientConfig_setDefault, UA_Client_connect, UA_Client_delete,
        UA_Client_disconnect, UA_Client_getConfig, UA_Client_new, UA_Client_readValueAttribute,
        UA_DataType, UA_DateTime, UA_DateTimeStruct, UA_DateTime_toStruct, UA_NodeId,
        UA_NodeId_clear, UA_NodeId_init, UA_Variant, UA_Variant_clear, UA_Variant_hasScalarType,
        UA_Variant_init, UA_NODEID_NUMERIC, UA_STATUSCODE_GOOD,
    };

    pub struct Client {
        client: *mut UA_Client,
    }

    impl Client {
        pub fn new() -> Self {
            let client = unsafe {
                let client = UA_Client_new();

                if client.is_null() {
                    panic!("UA_Client_new() returned `NULL`");
                }

                client
            };

            unsafe {
                let config = UA_Client_getConfig(client);

                if config.is_null() {
                    UA_Client_delete(client);

                    panic!("UA_Client_getConfig() returned `NULL`");
                }

                let result = UA_ClientConfig_setDefault(config);

                if result != UA_STATUSCODE_GOOD {
                    UA_Client_delete(client);

                    panic!("UA_ClientConfig_setDefault() returned `{result}`");
                }
            }

            Client { client }
        }

        pub fn connect(&mut self, endpoint_url: &str) -> Option<()> {
            let endpoint_url = ffi::CString::new(endpoint_url).ok()?;

            unsafe {
                let endpoint_url =
                    endpoint_url.as_bytes_with_nul() as *const _ as *const ffi::c_char;

                let result = UA_Client_connect(self.client, endpoint_url);

                if result == UA_STATUSCODE_GOOD {
                    Some(())
                } else {
                    None
                }
            }
        }

        pub fn disconnect(&mut self) -> Option<()> {
            unsafe {
                let result = UA_Client_disconnect(self.client);

                if result == UA_STATUSCODE_GOOD {
                    Some(())
                } else {
                    None
                }
            }
        }

        pub fn read_value_attribute(&mut self, node_id: &NodeId) -> Option<Variant> {
            let mut value = Variant::new();

            unsafe {
                let result =
                    UA_Client_readValueAttribute(self.client, node_id.node_id, &mut value.variant);

                if result == UA_STATUSCODE_GOOD {
                    Some(value)
                } else {
                    None
                }
            }
        }
    }

    impl Drop for Client {
        fn drop(&mut self) {
            unsafe {
                UA_Client_delete(self.client);
            }
        }
    }

    pub struct Variant {
        variant: UA_Variant,
    }

    impl Variant {
        pub fn new() -> Self {
            let variant = unsafe {
                let mut variant = MaybeUninit::uninit();

                UA_Variant_init(variant.as_mut_ptr());

                variant.assume_init()
            };

            Variant { variant }
        }

        pub fn has_scalar_type(&self, data_type: &DataType) -> bool {
            unsafe {
                let has_type = UA_Variant_hasScalarType(&self.variant, data_type.data_type);

                has_type
            }
        }

        pub fn into_date_time(self) -> Option<UA_DateTimeStruct> {
            if self.has_scalar_type(&types::DATE_TIME) {
                let value = self.variant.data as *const UA_DateTime;

                let value = unsafe {
                    let value = UA_DateTime_toStruct(*value);
                    value
                };

                Some(value)
            } else {
                None
            }
        }
    }

    impl Drop for Variant {
        fn drop(&mut self) {
            unsafe {
                UA_Variant_clear(&mut self.variant);
            }
        }
    }

    impl std::fmt::Debug for Variant {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // TODO: Revisit this.

            write!(f, "{:?}", self.variant)
        }
    }

    pub struct NodeId {
        node_id: UA_NodeId,
    }

    impl NodeId {
        pub fn new() -> Self {
            let node_id = unsafe {
                let mut node_id = MaybeUninit::uninit();

                UA_NodeId_init(node_id.as_mut_ptr());

                node_id.assume_init()
            };

            NodeId { node_id }
        }

        pub fn numeric(ns_index: u16, identifier: u32) -> Self {
            let node_id = unsafe {
                let node_id = UA_NODEID_NUMERIC(ns_index, identifier);

                node_id
            };

            NodeId { node_id }
        }
    }

    impl Drop for NodeId {
        fn drop(&mut self) {
            unsafe {
                UA_NodeId_clear(&mut self.node_id);
            }
        }
    }

    pub struct DataType {
        data_type: *const UA_DataType,
    }

    unsafe impl Sync for DataType {}

    pub mod types {
        use open62541_sys::{UA_TYPES, UA_TYPES_DATETIME};

        use super::DataType;

        pub static DATE_TIME: DataType = DataType {
            data_type: unsafe { &UA_TYPES[UA_TYPES_DATETIME as usize] },
        };
    }
}
