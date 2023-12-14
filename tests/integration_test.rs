use open62541_sys::{UA_Client_delete, UA_Client_new};

#[test]
fn create_and_destroy_client() {
    let client = unsafe { UA_Client_new() };
    unsafe { UA_Client_delete(client) };
}
