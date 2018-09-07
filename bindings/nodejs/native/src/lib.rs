/*!

NodeJS bindings.

*/

extern crate gutenberg_post_parser;
#[macro_use] extern crate neon;
extern crate serde_json;
extern crate neon_serde;

use gutenberg_post_parser::ast::Node;
use neon::js::{JsArray, JsNull, JsString, JsObject, Object};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::vm::{Call, JsResult, Throw};
use std::ops::DerefMut;
use std::str;

macro_rules! to_str (
    ($slice:expr) => (
        unsafe { str::from_utf8_unchecked($slice) }
    )
);

fn root(call: Call) -> JsResult<JsArray> {
    let scope = call.scope;
    let arguments = call.arguments;

    let input = arguments.require(scope, 0)?.check::<JsString>()?.value();
    let mut output: Handle<JsArray>;

    if let Ok((_remaining, nodes)) = gutenberg_post_parser::root(input.as_bytes()) {
        output = JsArray::new(scope, nodes.len() as u32);

        let raw_output = output.deref_mut();

        for (index, node) in nodes.iter().enumerate() {
            raw_output.set(
                index as u32,
                into_js_object(&node, scope)?
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

fn into_js_object<'a, 'b, S: Scope<'b>>(node: &Node<'a>, scope: &mut S) -> JsResult<'b, JsObject> {
    let output = JsObject::new(scope);

    match *node {
        Node::Block { name, attributes, ref children } => {
            // Name.
            output.set(
                "blockName",
                JsString::new_or_throw(
                    scope,
                    &format!(
                        "{}/{}",
                        to_str!(name.0),
                        to_str!(name.1)
                    )
                )?
            )?;

            // Attributes.
            output.set(
                "attrs",
                if let Some(attributes) = attributes {
                    let json =
                        serde_json::from_slice::<serde_json::Value>(attributes)
                        .map_err(|_| Throw)?;

                    neon_serde::to_value(scope, &json)?
                } else {
                    JsNull::new().upcast()
                }
            )?;

            // Inner blocks.
            let number_of_blocks =
                children
                .iter()
                .fold(
                    0u32,
                    |accumulator, node| {
                        accumulator +
                            match node {
                                Node::Block { .. } => 1,
                                _                  => 0
                            }
                    }
                );
            let mut blocks = JsArray::new(scope, number_of_blocks);
            let mut phrases = String::new();

            {
                let raw_blocks = blocks.deref_mut();
                let mut index = 0u32;

                for node in children {
                    match node {
                        Node::Block { .. } => {
                            raw_blocks.set(
                                index,
                                into_js_object(&node, scope)?
                            )?;

                            index += 1;
                        },

                        Node::Phrase(phrase) => {
                            phrases.push_str(to_str!(phrase));
                        }
                    }
                }
            }

            let phrases = JsString::new_or_throw(scope, phrases.as_str())?;

            output.set("innerBlocks", blocks)?;
            output.set("innerHTML", phrases)?;
        },

        Node::Phrase(phrase) => {
            output.set(
                "attrs",
                JsObject::new(scope)
            )?;
            output.set(
                "innerHTML",
                JsString::new_or_throw(
                    scope,
                    to_str!(phrase)
                )?
            )?;
        }
    }

    Ok(output)
}
