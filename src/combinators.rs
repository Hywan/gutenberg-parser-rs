/*!

Additional combinators specifically tailored for this parser.
 
Warning: It's likely the combinators are public to the crate only, and
thus can be absent from the public documentation.

*/

use super::Input;
use nom::IResult;
use std::vec::Vec;

/// `take_until_terminated(S, C)` is like `take_until` but with a
/// lookahead combinator `C`. It's not similar to
/// `terminated!(take_until(S, peek!(C)))` because it loops over the
/// input until `C` is true.
#[macro_export]
macro_rules! take_until_terminated (
    (_ $input:expr, $substr:expr, $consume:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use ::nom::{
                ErrorKind,
                FindSubstring,
                IResult,
                Needed,
                Slice,
                need_more_err
            };

            let input = $input;
            let substr_length = $substr.len();
            let mut index = 0;
            let mut result: Option<IResult<_, _>> = None;

            while let Some(next_index) = input.slice(index..).find_substring($substr) {
                match $submac!(input.slice(index + next_index + substr_length..), $($args)*) {
                    Ok(_) => {
                        let separator = if $consume {
                            index + next_index + substr_length
                        } else {
                            index + next_index
                        };

                        result = Some(Ok((input.slice(separator..), input.slice(0..separator))));

                        break;
                    },

                    _ => {
                        index += next_index + 1;
                    }
                }
            }

            if let Some(result) = result {
                result
            } else {
                need_more_err(input, Needed::Unknown, ErrorKind::Custom(42u32))
            }
        }
    );

    ($input:expr, $substr:expr, $submac:ident!( $($args:tt)* )) => (
        take_until_terminated!(_ $input, $substr, false, $submac!($($args)*));
    );

    ($input:expr, $substr:expr, $f:expr) => (
        take_until_terminated!(_ $input, $substr, false, call!($f));
    );
);

/// `take_until_terminated_and_consume(S, C)` is similar to
/// `take_until_terminated` but it consumes `S`.
#[macro_export]
macro_rules! take_until_terminated_and_consume (
    ($input:expr, $substr:expr, $submac:ident!( $($args:tt)* )) => (
        take_until_terminated!(_ $input, $substr, true, $submac!($($args)*));
    );

    ($input:expr, $substr:expr, $f:expr) => (
        take_until_terminated!(_ $input, $substr, true, call!($f));
    );
);

/// `fold_into_vector_many0!(I -> IResult<I,O>, R) => I -> IResult<I, R>`
/// is a wrapper around `fold_many0!` specifically designed for vectors.
///
/// This is strictly equivalent to `fold_many0!(submacro!(…),
/// Vec::new(), fold_into_vector)` but it shrinks the capacity of the
/// vector to fit the current length.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate nom;
/// #[macro_use] extern crate gutenberg_post_parser;
///
/// # fn main() {
/// named!(
///     test<Vec<&[u8]>>,
///     fold_into_vector_many0!(
///         tag!("abc"),
///         Vec::new()
///     )
/// );
///
/// if let Ok((_, vector)) = test(b"abcabcabc") {
///     assert_eq!(vector.capacity(), vector.len());
/// }
/// # }
/// ```
#[macro_export]
macro_rules! fold_into_vector_many0(
    ($input:expr, $submacro:ident!($($arguments:tt)*), $init:expr) => (
        {
            let result = fold_many0!(
                $input,
                $submacro!($($arguments)*),
                $init,
                $crate::combinators::fold_into_vector
            );

            if let Ok((remaining, mut output)) = result {
                output.shrink_to_fit();

                Ok((remaining, output))
            } else {
                result
            }
        }
    );

    ($input:expr, $function:expr, $init:expr) => (
        fold_many0!($input, call!($function), $init);
    );
);

/// Helper to fold an item into a vector.
pub fn fold_into_vector<I>(mut accumulator: Vec<I>, item: I) -> Vec<I> {
    accumulator.push(item);

    accumulator
}

/// Check whether a character is in the set of alphanumeric extended
/// characters, i.e. `[a-z0-9_-]`.
pub(crate) fn is_alphanumeric_extended(chr: u8) -> bool {
    (chr >= 0x61 && chr <= 0x7a) || (chr >= 0x30 && chr <= 0x39) || chr == b'_' || chr == b'-'
}

/// The `id` combinator consumes the entire given input as the output.
pub(crate) fn id(input: Input) -> IResult<Input, Input> {
    Ok((&b""[..], input))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_take_until_terminated_ok() {
        named!(
            parser,
            take_until_terminated_and_consume!(
                "d",
                tag!("c")
            )
        );

        let input = &b"abcdcba"[..];
        let output: ::nom::IResult<_, _> = Ok((&b"cba"[..], &b"abcd"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_until_terminated_ok_at_position_0() {
        named!(
            parser,
            take_until_terminated_and_consume!(
                "a",
                tag!("b")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"bcdcba"[..], &b"a"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_until_terminated_ok_at_position_eof_minus_one() {
        named!(
            parser,
            take_until_terminated_and_consume!(
                "b",
                tag!("a")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"a"[..], &b"abcdcb"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_until_terminated_ok_with_multiple_substring() {
        named!(
            parser,
            take_until_terminated_and_consume!(
                "c",
                tag!("b")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"ba"[..], &b"abcdc"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_until_terminated_error() {
        named!(
            parser,
            take_until_terminated_and_consume!(
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
    fn test_take_until_terminated_optional() {
        named!(
            parser<&[u8], Option<&[u8]>>,
            opt!(
                complete!(
                    take_until_terminated_and_consume!(
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
