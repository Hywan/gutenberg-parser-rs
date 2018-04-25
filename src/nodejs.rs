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

        let raw_output = output.deref_mut();

        for (index, block) in blocks.iter().enumerate() {
            raw_output.set(
                index as u32,
                block.into_js_object(scope)?
            )?;
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

impl<'a> Block<'a> {
    fn into_js_object<'b, S: Scope<'b>>(&self, scope: &mut S) -> JsResult<'b, JsObject> {
        let output = JsObject::new(scope);

        output.set(
            "name",
            JsString::new_or_throw(
                scope,
                &format!(
                    "{}/{}",
                    unsafe { str::from_utf8_unchecked(self.name.0) },
                    unsafe { str::from_utf8_unchecked(self.name.1) }
                )
            )?
        )?;
        output.set(
            "attributes",
            if let Some(attributes) = self.attributes {
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

        let mut inner_blocks = JsArray::new(scope, self.inner_blocks.len() as u32);

        {
            let raw_inner_blocks = inner_blocks.deref_mut();

            for (index, block) in self.inner_blocks.iter().enumerate() {
                raw_inner_blocks.set(
                    index as u32,
                    block.into_js_object(scope)?
                )?;
            }
        }

        output.set(
            "inner_blocks",
            inner_blocks
        )?;

        Ok(output)
    }
}
