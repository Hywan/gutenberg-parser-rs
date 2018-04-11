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

#[macro_use]
mod combinators;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;




#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    name: (&'a [u8], &'a [u8]),
    attributes: Option<json::Value>,
    inner_blocks: Vec<Block<'a>>
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
    pub block_list<&[u8], Vec<Block>>,
    many0!(
        preceded!(
            anything_but_block,
            block
        )
    )
);

named_attr!(
    #[doc="foo"],
    block<&[u8], Block>,
    alt_complete!(
        block_balanced |
        block_void
    )
);

named!(
    anything_but_block,
    complete!(take_until!("<!--"))
);

named_attr!(
    #[doc="foo"],
    block_balanced<&[u8], Block>,
    do_parse!(
        tag!("<!--") >>
        opt!(whitespaces) >>
        tag!("wp:") >>
        name: block_name >>
        whitespaces >>
        attributes: opt!(block_attributes) >>
        opt!(whitespaces) >>
        tag!("-->") >>
        inner_blocks: many0!(
            preceded!(
                anything_but_block,
                block
            )
        ) >>
        preceded!(
            anything_but_block,
            tag!("<!--")
        ) >>
        opt!(whitespaces) >>
        tag!("/wp:") >>
        _closing_name: block_name >>
        opt!(whitespaces) >>
        tag!("-->") >>
        (
            // @todo: Need to check that `closing_name` is equal to `name`.
            Block {
                name: name,
                attributes: attributes,
                inner_blocks: inner_blocks
            }
        )
    )
);

named_attr!(
    #[doc="foo"],
    block_void<&[u8], Block>,
    do_parse!(
        tag!("<!--") >>
        opt!(whitespaces) >>
        tag!("wp:") >>
        name: block_name >>
        whitespaces >>
        attributes: opt!(block_attributes) >>
        opt!(whitespaces) >>
        tag!("/-->") >>
        (
            Block {
                name: name,
                attributes: attributes,
                inner_blocks: vec![]
            }
        )
    )
);

named_attr!(
    #[doc="foo"],
    block_name<&[u8], (&[u8], &[u8])>,
    alt!(
        namespaced_block_name |
        core_block_name
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
    block_name_part,
    recognize!(
        pair!(
            is_a!("abcdefghijklmnopqrstuvwxyz"),
            take_while!(combinators::is_alphanumeric_extended)
        )
    )
);

named_attr!(
    #[doc="foo"],
    block_attributes<&[u8], json::Value>,
    map_res!(
        preceded!(
            peek!(tag!("{")),
            take_till_terminated!(
                "}",
                preceded!(
                    opt!(whitespaces),
                    alt_complete!(
                        tag!("/-->") |
                        tag!("-->")
                    )
                )
            )
        ),
        |json| {
            json::de::from_slice(json)
        }
    )
);

named_attr!(
    #[doc="foo"],
    whitespaces,
    is_a!(" \n\r\t")
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
        let input = &b"abc <!-- wp:foo --><!-- wp:bar /--> def <!-- /wp:foo --> ghi"[..];
        let output = Ok(
            (
                &b" ghi"[..],
                vec![
                    Block {
                        name: (&b"core"[..], &b"foo"[..]),
                        attributes: None,
                        inner_blocks: vec![
                            Block {
                                name: (&b"core"[..], &b"bar"[..]),
                                attributes: None,
                                inner_blocks: vec![]
                            }
                        ]
                    }
                ]
            )
        );

        assert_eq!(block_list(input), output);
    }

    #[test]
    fn test_block_balanced_default_namespace_without_attributes() {
        let input = &b"<!-- wp:foo --><!-- /wp:foo -->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                inner_blocks: vec![]
            }
        ));

        assert_eq!(block_balanced(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_coerce_namespace_without_attributes() {
        let input = &b"<!-- wp:ns/foo --><!-- /wp:ns/foo -->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: None,
                inner_blocks: vec![]
            }
        ));

        assert_eq!(block_balanced(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_coerce_namespace_with_attributes() {
        let input = &b"<!-- wp:ns/foo {\"abc\": \"xyz\"} --><!-- /wp:ns/foo -->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: Some(json!({"abc": "xyz"})),
                inner_blocks: vec![]
            }
        ));

        assert_eq!(block_balanced(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_with_children() {
        let input = &b"<!-- wp:foo --><!-- wp:bar {\"abc\": true} /--><!-- wp:baz --><!-- wp:qux /--><!-- /wp:baz --><!-- /wp:foo -->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                inner_blocks: vec![
                    Block {
                        name: (&b"core"[..], &b"bar"[..]),
                        attributes: Some(json!({"abc": true})),
                        inner_blocks: vec![]
                    },
                    Block {
                        name: (&b"core"[..], &b"baz"[..]),
                        attributes: None,
                        inner_blocks: vec![
                            Block {
                                name: (&b"core"[..], &b"qux"[..]),
                                attributes: None,
                                inner_blocks: vec![]
                            }
                        ]
                    }
                ]
            }
        ));

        assert_eq!(block_balanced(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_with_phrasing_children() {
        let input = &b"<!-- wp:foo --> abc <!-- wp:bar {\"abc\": true} /--> def <!-- wp:baz --> ghi <!-- wp:qux /--> jkl <!-- /wp:baz --> mno <!-- /wp:foo -->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                inner_blocks: vec![
                    Block {
                        name: (&b"core"[..], &b"bar"[..]),
                        attributes: Some(json!({"abc": true})),
                        inner_blocks: vec![]
                    },
                    Block {
                        name: (&b"core"[..], &b"baz"[..]),
                        attributes: None,
                        inner_blocks: vec![
                            Block {
                                name: (&b"core"[..], &b"qux"[..]),
                                attributes: None,
                                inner_blocks: vec![]
                            }
                        ]
                    }
                ]
            }
        ));

        assert_eq!(block_balanced(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_void_default_namespace_without_attributes() {
        let input = &b"<!-- wp:foo /-->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                inner_blocks: vec![]
            }
        ));

        assert_eq!(block_void(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_void_coerce_namespace_without_attributes() {
        let input = &b"<!-- wp:ns/foo /-->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: None,
                inner_blocks: vec![]
            }
        ));

        assert_eq!(block_void(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_void_coerce_namespace_with_attributes() {
        let input = &b"<!-- wp:ns/foo {\"abc\": \"xyz\"} /-->"[..];
        let output = Ok((
            &b""[..],
            Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: Some(json!({"abc": "xyz"})),
                inner_blocks: vec![]
            }
        ));

        assert_eq!(block_void(input), output);
        assert_eq!(block(input), output);
    }

    #[test]
    fn test_namespaced_block_name() {
        let input = &b"foo_bar/baz42 x"[..];
        let output = Ok((&b" x"[..], (&b"foo_bar"[..], &b"baz42"[..])));

        assert_eq!(namespaced_block_name(input), output);
        assert_eq!(block_name(input), output);
    }

    #[test]
    fn test_core_block_name() {
        let input = &b"foo x"[..];
        let output = Ok((&b" x"[..], (&b"core"[..], &b"foo"[..])));

        assert_eq!(core_block_name(input), output);
        assert_eq!(block_name(input), output);
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
        let output = Ok((&b" \t\r\n-->"[..], json!({"foo": true})));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_object_with_auto_close() {
        let input = &b"{\"foo\": \"bar\", \"baz\": [1, 2]}/-->"[..];
        let output = Ok((&b"/-->"[..], json!({"foo": "bar", "baz": [1, 2]})));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_whitespaces() {
        let input = &b" \n\r\t xyz"[..];
        let output = Ok((&b"xyz"[..], &b" \n\r\t "[..]));

        assert_eq!(whitespaces(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok() {
        named!(
            parser,
            take_till_terminated!(
                "d",
                tag!("c")
            )
        );

        let input = &b"abcdcba"[..];
        let output: ::nom::IResult<_, _> = Ok((&b"cba"[..], &b"abcd"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok_at_position_0() {
        named!(
            parser,
            take_till_terminated!(
                "a",
                tag!("b")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"bcdcba"[..], &b"a"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok_at_position_eof_minus_one() {
        named!(
            parser,
            take_till_terminated!(
                "b",
                tag!("a")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"a"[..], &b"abcdcb"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok_with_multiple_substring() {
        named!(
            parser,
            take_till_terminated!(
                "c",
                tag!("b")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"ba"[..], &b"abcdc"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_error() {
        named!(
            parser,
            take_till_terminated!(
                "a",
                tag!("z")
            )
        );

        use ::nom::{ErrorKind, Needed, need_more_err};

        let input = &b"abcdcba"[..];
        let output = need_more_err(input, Needed::Unknown, ErrorKind::Custom(42u32));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_optional() {
        named!(
            parser<&[u8], Option<&[u8]>>,
            opt!(
                complete!(
                    take_till_terminated!(
                        "a",
                        tag!("z")
                    )
                )
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((input, None));

        assert_eq!(parser(input), output);
    }
}
