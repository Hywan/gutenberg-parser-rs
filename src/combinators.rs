/*!

Additional combinators specifically tailored for this parser.
 
Warning: It's likely the combinators are public to the crate only, and
thus can be absent from the public documentation.

*/

#[cfg(feature = "wasm")] use alloc::Vec;

/// `take_till_terminated(S, C)` is a like `take_till` but with a lookahead
/// combinator `C`.
#[macro_export]
macro_rules! take_till_terminated (
    ($input:expr, $substr:expr, $submac:ident!( $($args:tt)* )) => (
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
            let mut index = 0;
            let mut result: Option<IResult<_, _>> = None;

            while let Some(next_index) = input.slice(index..).find_substring($substr) {
                match $submac!(input.slice(index + next_index + 1..), $($args)*) {
                    Ok(_) => {
                        result = Some(Ok((input.slice(index + next_index + 1..), input.slice(0..index + next_index + 1))));

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

    ($input:expr, $substr:expr, $f:expr) => {
        take_till_terminated!($input, $substr, call!($f));
    }
);

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

pub(crate) fn fold_into_vector<I>(mut accumulator: Vec<I>, item: I) -> Vec<I> {
    accumulator.push(item);

    accumulator
}

pub(crate) fn is_alphanumeric_extended(chr: u8) -> bool {
    (chr >= 0x61 && chr <= 0x7a) || (chr >= 0x30 && chr <= 0x39) || chr == b'_' || chr == b'-'
}

#[cfg(test)]
mod tests {
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
