/*!

# The Gutenberg post parser.

[Gutenberg] is a new post editor for the [WordPress] ecosystem. A post
has always been HTML, and it continues to be. The difference is that
the HTML is annotated. Like most annotation language, it is located in
comments, like this:

```html
<h1>Famous post</h1>

<!-- wp:component {attributes: "as JSON"} -->
lorem ipsum
<!-- /wp:component -->
```

The parser analyses a post and generates an Abstract Syntax Tree (AST) of it.

## Platforms and bindings

The parser aims at being used on different platforms, such as: Web
within multiple browsers, Web applications like [Electron], native
applications like macOS, iOS, Windows, Linux etc.

Thus, the parser can be compiled as a static library, can be embedded
in any Rust projects, can be compiled to [WebAssembly], and soon more.

This project uses [Justfile] as an alternative to Makefile. Every
following command will use `just`, you might consider to install
it. To learn about all the commands, just `just --list`.

### Static library

To compile the parser to a static library, run:

```sh
$ just build-library
$ ls target/release/
```

### WebAssembly

To compile the parser to a [WebAssembly] file, run:

```sh
$ just build-wasm
$ open bindings/wasm/index.html # for a demonstration
```

## Performance and guarantee

The parser guarantees to never copy the data in memory, which makes it
fast and memory efficient.

## License

The license is a classic `BSD-3-Clause`:

> New BSD License
>
> Copyright ©, Ivan Enderlin. All rights reserved.
>
> Redistribution and use in source and binary forms, with or without
> modification, are permitted provided that the following conditions are met:
>
>   * Redistributions of source code must retain the above copyright
>     notice, this list of conditions and the following disclaimer.
>
>   * Redistributions in binary form must reproduce the above copyright
>     notice, this list of conditions and the following disclaimer in the
>     documentation and/or other materials provided with the distribution.
>
>   * Neither the name of this project nor the names of its contributors may be
>     used to endorse or promote products derived from this software without
>     specific prior written permission.
>
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
> AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
> IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
> ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
> LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
> CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
> SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
> INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
> CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
> ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
> POSSIBILITY OF SUCH DAMAGE.

[Gutenberg]: https://github.com/WordPress/gutenberg/
[WordPress]: https://wordpress.org/
[Electron]: https://github.com/electron/
[Justfile]: https://github.com/casey/just/
[WebAssembly]: http://webassembly.org/

*/


#![cfg_attr(feature = "wasm", no_std)]
#![
    cfg_attr(
        feature = "wasm",
        feature(
            proc_macro,
            wasm_custom_section,
            wasm_import_module,
            global_allocator,
            alloc,
            core_intrinsics,
            lang_items
        )
    )
]


#[cfg(feature = "wasm")] #[macro_use] extern crate alloc;
#[macro_use] extern crate nom;
#[cfg(feature = "wasm")] extern crate wee_alloc;
#[cfg(feature = "nodejs")] #[macro_use] extern crate neon;


#[cfg(feature = "wasm")]
use alloc::Vec;


// Export modules.
pub mod ast;
#[macro_use] pub mod combinators;
pub mod parser;
#[cfg(feature = "wasm")] pub mod wasm;
#[cfg(feature = "nodejs")] pub mod nodejs;


// Configure `wee_alloc`.
#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Represent the type of a parser input element. See
/// [`Input`](./type.Input.html) for more information.
pub type InputElement = u8;

/// Represent the type of a parser input.
///

/// The parser does not analyse a `String` nor a `&str`, but a slice
/// of bytes `&[u8]`. One of the consequence is that there is no UTF-8
/// validation (Rust guarantees that all strings are valid UTF-8
/// data). There is many arguments for this decision, one of them is
/// that the post format are likely to contain JSON encoded data, and
/// JSON has a weird encoding format for strings (e.g. surrogate
/// pairs), which might not be compatible with UTF-8. Other arguments
/// are mostly related to memory efficiency.
pub type Input<'a> = &'a [InputElement];

/// The `root` function represents the axiom of the grammar, i.e. the top rule.
///
/// This is the main function to call to parse a traditional post.
///
/// # Examples
///
/// In this example, one might notice that the output is a pair, where
/// the left side contains the remaining data (i.e. data that have not
/// been parsed, because the parser has stopped), and the right side
/// contains the Abstract Syntax Tree (AST).
///
/// The left side should ideally always be empty.
///
/// ```
/// extern crate gutenberg_post_parser;
///
/// use gutenberg_post_parser::{root, ast::Block};
///
/// let input = &b"<!-- wp:foo {\"bar\": true} /-->"[..];
/// let output = Ok(
///     (
///         // The remaining data.
///         &b""[..],
///
///         // The Abstract Syntax Tree.
///         vec![
///             Block {
///                 name: (&b"core"[..], &b"foo"[..]),
///                 attributes: Some(&b"{\"bar\": true}"[..]),
///                 inner_blocks: vec![]
///             }
///         ]
///     )
/// );
///
/// assert_eq!(root(input), output);
/// ```
pub fn root(input: Input) -> Result<(Input, Vec<ast::Block>), nom::Err<Input>> {
    parser::block_list(input)
}
