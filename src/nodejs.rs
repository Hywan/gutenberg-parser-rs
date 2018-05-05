/*!

NodeJS bindings.

*/

use super::ast::Node;
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

impl<'a> Node<'a> {
    /// The form of the object matches the expectation of the
    /// Gutenberg project, it explains why the keys differ from the
    /// `Block` structure.
    fn into_js_object<'b, S: Scope<'b>>(&self, scope: &mut S) -> JsResult<'b, JsObject> {
        let output = JsObject::new(scope);

        match self {
            Node::Block { name, attributes, children } => {
                // Name.
                output.set(
                    "blockName",
                    JsString::new_or_throw(
                        scope,
                        &format!(
                            "{}/{}",
                            unsafe { str::from_utf8_unchecked(name.0) },
                            unsafe { str::from_utf8_unchecked(name.1) }
                        )
                    )?
                )?;

                // Attributes.
                output.set(
                    "attrs",
                    if let Some(attributes) = attributes {
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

                // Children.
                let mut children_ = JsArray::new(scope, children.len() as u32);

                {
                    let raw_children = children_.deref_mut();

                    for (index, block) in children.iter().enumerate() {
                        raw_children.set(
                            index as u32,
                            block.into_js_object(scope)?
                        )?;
                    }
                }

                output.set(
                    "children",
                    children_
                )?;
            },

            Node::Phrase(phrase) => {
            }
        }

        Ok(output)
    }
}
