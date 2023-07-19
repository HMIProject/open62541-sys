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
    use std::ptr;

    let mut args = std::env::args().skip(1);
    let remote = args.next().unwrap_or_else(|| {
        eprintln!(
            "Usage: {} URL [username] [password]",
            std::env::args().next().unwrap()
        );
        std::process::exit(1);
    });
    let username = args.next();
    let password = args.next();

    unsafe {
        let credentials = match (&username, &password) {
            (Some(username), Some(password)) => format!("username {username}, password {password}"),
            _ => format!("anonymous"),
        };
        println!("Connecting to {remote} ({credentials}) …");
        let client = open62541_sys::UA_Client_new();
        let config = open62541_sys::UA_Client_getConfig(client);
        open62541_sys::UA_ClientConfig_setDefault(config);
        unsafe extern "C" fn state_callback(
            _client: *mut open62541_sys::UA_Client,
            _channel_state: open62541_sys::UA_SecureChannelState,
            _session_state: open62541_sys::UA_SessionState,
            _connect_status: open62541_sys::UA_StatusCode,
        ) {
            // println!("Client state changed: {channel_state} {session_state} {connect_status}");
        }
        (*config).stateCallback = Some(state_callback);
        let retval = match (username, password) {
            (Some(username), Some(password)) => open62541_sys::UA_Client_connectUsername(
                client,
                ffi::CString::new(remote.clone())
                    .unwrap()
                    .as_bytes_with_nul() as *const _ as *const ffi::c_char,
                ffi::CString::new(username).unwrap().as_bytes_with_nul() as *const _
                    as *const ffi::c_char,
                ffi::CString::new(password).unwrap().as_bytes_with_nul() as *const _
                    as *const ffi::c_char,
            ),
            _ => open62541_sys::UA_Client_connect(
                client,
                ffi::CString::new(remote.clone())
                    .unwrap()
                    .as_bytes_with_nul() as *const _ as *const ffi::c_char,
            ),
        };
        if retval != open62541_sys::UA_STATUSCODE_GOOD {
            open62541_sys::UA_Client_delete(client);
            return;
        }
        println!("Connected to {remote}");

        let node_id = open62541_sys::UA_NODEID_NUMERIC(
            0,
            open62541_sys::UA_NS0ID_SERVER_SERVERSTATUS_CURRENTTIME,
        );

        let mut value = MaybeUninit::uninit();
        open62541_sys::UA_Variant_init(value.as_mut_ptr());
        let mut value = value.assume_init();
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

        let mut sub_request = open62541_sys::UA_CreateSubscriptionRequest_default();
        unsafe extern "C" fn status_callback(
            _client: *mut open62541_sys::UA_Client,
            sub_id: open62541_sys::UA_UInt32,
            _sub_context: *mut ::std::os::raw::c_void,
            notification: *mut open62541_sys::UA_StatusChangeNotification,
        ) {
            println!(
                "Status of subscription {sub_id} changed: {}",
                (*notification).status
            );
        }
        unsafe extern "C" fn delete_callback_sub(
            _client: *mut open62541_sys::UA_Client,
            sub_id: open62541_sys::UA_UInt32,
            _sub_context: *mut ::std::os::raw::c_void,
        ) {
            println!("Subscription {sub_id} was deleted");
        }
        let retval = open62541_sys::UA_Client_Subscriptions_create(
            client,
            sub_request,
            ptr::null_mut(),
            Some(status_callback),
            Some(delete_callback_sub),
        );
        open62541_sys::UA_CreateSubscriptionRequest_clear(&mut sub_request);
        if retval.responseHeader.serviceResult != open62541_sys::UA_STATUSCODE_GOOD {
            open62541_sys::UA_Client_delete(client);
            return;
        }
        let sub_id = retval.subscriptionId;
        println!("Created subscription with ID {sub_id}");

        let mut mon_request = open62541_sys::UA_MonitoredItemCreateRequest_default(node_id);
        unsafe extern "C" fn change_callback(
            _client: *mut open62541_sys::UA_Client,
            sub_id: open62541_sys::UA_UInt32,
            _sub_context: *mut ::std::os::raw::c_void,
            mon_id: open62541_sys::UA_UInt32,
            _mon_context: *mut ::std::os::raw::c_void,
            value: *mut open62541_sys::UA_DataValue,
        ) {
            print!("Value of monitored item {mon_id} (at subscription {sub_id}) changed: ");
            let value = (*value).value;
            if open62541_sys::UA_Variant_hasScalarType(
                &value,
                &open62541_sys::UA_TYPES[open62541_sys::UA_TYPES_DATETIME as usize],
            ) {
                let raw_date = value.data as *const open62541_sys::UA_DateTime;
                let dts = open62541_sys::UA_DateTime_toStruct(*raw_date);
                println!("{dts:?}");
            }
        }
        unsafe extern "C" fn delete_callback_mon(
            _client: *mut open62541_sys::UA_Client,
            sub_id: open62541_sys::UA_UInt32,
            _sub_context: *mut ::std::os::raw::c_void,
            mon_id: open62541_sys::UA_UInt32,
            _mon_context: *mut ::std::os::raw::c_void,
        ) {
            println!("Monitored item {mon_id} (at subscription {sub_id}) was deleted");
        }
        let retval = open62541_sys::UA_Client_MonitoredItems_createDataChange(
            client,
            sub_id,
            open62541_sys::UA_TimestampsToReturn_UA_TIMESTAMPSTORETURN_BOTH,
            mon_request,
            ptr::null_mut(),
            Some(change_callback),
            Some(delete_callback_mon),
        );
        open62541_sys::UA_MonitoredItemCreateRequest_clear(&mut mon_request);
        if retval.statusCode != open62541_sys::UA_STATUSCODE_GOOD {
            open62541_sys::UA_Client_delete(client);
            return;
        }
        let mon_id = retval.monitoredItemId;
        println!("Created monitored item with ID {mon_id} (at subscription {sub_id})");

        loop {
            let retval = open62541_sys::UA_Client_run_iterate(client, 250);
            if retval != open62541_sys::UA_STATUSCODE_GOOD {
                break;
            }
        }

        let mut sub_request = MaybeUninit::uninit();
        open62541_sys::UA_DeleteSubscriptionsRequest_init(sub_request.as_mut_ptr());
        let mut sub_request = sub_request.assume_init();
        let mut req_sub_ids = [sub_id];
        sub_request.subscriptionIds = req_sub_ids.as_mut_ptr();
        sub_request.subscriptionIdsSize = req_sub_ids.len();
        let retval = open62541_sys::UA_Client_Subscriptions_delete(client, sub_request);
        sub_request.subscriptionIdsSize = 0;
        sub_request.subscriptionIds = ptr::null_mut();
        open62541_sys::UA_DeleteSubscriptionsRequest_clear(&mut sub_request);
        if retval.responseHeader.serviceResult != open62541_sys::UA_STATUSCODE_GOOD {
            open62541_sys::UA_Client_delete(client);
            return;
        }
        println!("Deleted subscription with ID {sub_id}");

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
