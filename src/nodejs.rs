/*!

NodeJS bindings.

*/

use neon::vm::{Call, JsResult};
use neon::js::JsNumber;

fn hello(call: Call) -> JsResult<JsNumber> {
    Ok(JsNumber::new(call.scope, 43f64))
}

register_module!(
    module,
    {
        module.export("hello", hello)
    }
);
