#![feature(proc_macro, wasm_custom_section, wasm_import_module, global_allocator)]

#[macro_use]
extern crate nom;
//extern crate wasm_bindgen;
extern crate wee_alloc;
extern crate regex;
#[cfg(test)] #[macro_use] extern crate serde_json;
#[cfg(not(test))] extern crate serde_json;

//use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
use serde_json as json;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

fn is_alphanumeric_extended(chr: u8) -> bool {
    (chr >= 0x61 && chr <= 0x7a) || (chr >= 0x30 && chr <= 0x39) || chr == b'_' || chr == b'-'
}

/// Test
//#[wasm_bindgen]
pub fn root(input: &str) -> bool {
    if let Ok(_) = block_list(input.as_bytes()) {
        true
    } else {
        false
    }
}

named_attr!(
    #[doc="Test"],
    block_list,
    tag!("<!-- wp:foo /-->")
);

named_attr!(
    #[doc="foo"],
    core_block_name<&[u8], (&[u8], &[u8])>,
    map_res!(
        block_name_part,
        |block_name_part| -> Result<(&[u8], &[u8]), ()> {
            Ok((&b"core"[..], block_name_part))
        }
    )
);

named_attr!(
    #[doc="foo"],
    namespaced_block_name<&[u8], (&[u8], &[u8])>,
    tuple!(
        block_name_part,
        preceded!(
            tag!("/"),
            block_name_part
        )
    )
);

named_attr!(
    #[doc="foo"],
    block_name_part,
    recognize!(
        pair!(
            is_a!("abcdefghijklmnopqrstuvwxyz"),
            take_while!(is_alphanumeric_extended)
        )
    )
);

named_attr!(
    #[doc="foo"],
    block_attributes<&[u8], json::Value>,
    map_res!(
        preceded!(
            peek!(tag!("{")),
            take_until!("-->")
        ),
        |json| {
            json::de::from_slice(json)
        }
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        assert_eq!(root("<!-- wp:foo /-->"), true);
    }

    #[test]
    fn test_block_list() {
        let input = &b"<!-- wp:foo /-->"[..];
        let output = Ok((&b""[..], input));

        assert_eq!(block_list(input), output);
    }

    #[test]
    fn test_core_block_name() {
        let input = &b"foo x"[..];
        let output = Ok((&b" x"[..], (&b"core"[..], &b"foo"[..])));

        assert_eq!(core_block_name(input), output);
    }

    #[test]
    fn test_namespaced_block_name() {
        let input = &b"foo_bar/baz42 x"[..];
        let output = Ok((&b" x"[..], (&b"foo_bar"[..], &b"baz42"[..])));

        assert_eq!(namespaced_block_name(input), output);
    }

    #[test]
    fn test_block_name_part_shortest() {
        let input = &b"a x"[..];
        let output = Ok((&b" x"[..], &b"a"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_name_part_only_alpha() {
        let input = &b"abc xyz"[..];
        let output = Ok((&b" xyz"[..], &b"abc"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_name_part_only_alphanumeric() {
        let input = &b"a0b1c xyz"[..];
        let output = Ok((&b" xyz"[..], &b"a0b1c"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_name_part() {
        let input = &b"a0b_1c- xyz"[..];
        let output = Ok((&b" xyz"[..], &b"a0b_1c-"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_attributes_simple_object() {
        let input = &b"{\"foo\": \"bar\"}-->"[..];
        let output = Ok((&b"-->"[..], json!({"foo": "bar"})));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_object() {
        let input = &b"{\"foo\": \"bar\", \"baz\": [1, 2]}-->"[..];
        let output = Ok((&b"-->"[..], json!({"foo": "bar", "baz": [1, 2]})));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_nested_objects() {
        let input = &b"{\"foo\": {\"bar\": \"baz\"} }-->"[..];
        let output = Ok((&b"-->"[..], json!({"foo": {"bar": "baz"}})));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_surrounded_by_spaces() {
        let input = &b"{\"foo\": true} \t\r\n-->"[..];
        let output = Ok((&b"-->"[..], json!({"foo": true})));

        assert_eq!(block_attributes(input), output);
    }
}
