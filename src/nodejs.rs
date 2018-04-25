/*!

NodeJS bindings.

*/

use neon::vm::{Call, JsResult};
use neon::js::JsNumber;

fn hello(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let arguments = call.arguments;

    let x = arguments.require(scope, 0)?.check::<JsNumber>()?.value();

    Ok(JsNumber::new(scope, x + 1f64))
}

register_module!(
    module,
    {
        module.export("hello", hello)
    }
);
