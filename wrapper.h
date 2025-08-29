#include <open62541/types.h>
#include <open62541/client.h>
#include <open62541/client_config_default.h>
#include <open62541/client_highlevel.h>
#include <open62541/client_highlevel_async.h>
#include <open62541/client_subscriptions.h>
#include <open62541/server.h>
#include <open62541/server_config_default.h>
#include <open62541/plugin/accesscontrol.h>
#include <open62541/plugin/accesscontrol_default.h>
#include <open62541/plugin/certificategroup.h>
#include <open62541/plugin/certificategroup_default.h>
#include <open62541/plugin/create_certificate.h>
#include <open62541/plugin/log.h>
#include <open62541/plugin/log_stdout.h>
#include <open62541/plugin/securitypolicy.h>
#include <open62541/plugin/securitypolicy_default.h>

// bindgen does not support non-trivial `#define` used for pointer constant. Use
// statically defined constant as workaround for now.
//
// See <https://github.com/rust-lang/rust-bindgen/issues/2426>.
extern const void *const RS_UA_EMPTY_ARRAY_SENTINEL;
