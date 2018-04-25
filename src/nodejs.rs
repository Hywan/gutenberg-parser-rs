/*!

NodeJS bindings.

*/

use super::ast::Block;
use neon::js::{JsArray, JsString, JsObject, Object};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::vm::{Call, JsResult};
use std::ops::DerefMut;
use std::str;

fn root(call: Call) -> JsResult<JsArray> {
    let scope = call.scope;
    let arguments = call.arguments;

    let input = arguments.require(scope, 0)?.check::<JsString>()?.value();
    let mut output: Handle<JsArray>;

    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
        output = JsArray::new(scope, blocks.len() as u32);

        {
            let raw_output = output.deref_mut();

            for (index, block) in blocks.iter().enumerate() {
                let item = JsObject::new(scope);
                item.set(
                    "name",
                    JsString::new_or_throw(
                        scope,
                        &format!(
                            "{}/{}",
                            unsafe { str::from_utf8_unchecked(block.name.0) },
                            unsafe { str::from_utf8_unchecked(block.name.1) }
                        )
                    )?
                )?;
                item.set(
                    "attributes",
                    if let Some(attributes) = block.attributes {
                        JsString::new_or_throw(
                            scope,
                            unsafe { str::from_utf8_unchecked(attributes) }
                        )?
                    } else {
                        JsString::new_or_throw(
                            scope,
                            "{}"
                        )?
                    }
                )?;
                item.set(
                    "inner_blocks",
                    JsArray::new(scope, 0)
                )?;

                raw_output.set(index as u32, item)?;
            }
        }
    } else {
        output = JsArray::new(scope, 0u32);
    }

    Ok(output)
}

register_module!(
    module,
    {
        module.export("root", root)
    }
);
