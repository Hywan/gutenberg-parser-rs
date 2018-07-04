/*!

# The Gutenberg post parser

[Gutenberg] is a new post editor for the [WordPress] ecosystem. A post
has always been HTML, and it continues to be. The difference is that
the HTML is now annotated. Like most annotation languages, it is
located in comments, like this:

```html
<h1>Famous post</h1>

<!-- wp:component {attributes: "as JSON"} -->
lorem ipsum
<!-- /wp:component -->
```

The parser analyses a post and generates an Abstract Syntax Tree (AST)
of it. The AST is then accessible to many languages through bindings.

## Platforms and bindings, aka targets

The parser aims at being used on different platforms, such as: the Web
(within multiple browsers), Web applications like [Electron], native
applications like macOS, iOS, Windows, Linux etc.

Thus, the parser can be compiled as:

  * A [binary](#binary),
  * A [static library](#static-library),
  * Can be embedded in any Rust projects,
  * A [WebAssembly binary](#webassembly),
  * An [ASM.js module](#asmjs),
  * A [NodeJS native module](#nodejs),
  * A [C header](#c),
  * A [PHP extension](#php),
  * And soon more.

This project uses [Justfile] as an alternative to Makefile. Every
following command will use `just`, you might consider to install
it. To learn about all the commands, just `just --list`.

**Note**: Right now, this project needs `rustc` nightly to compile the
WASM target. This target should switch to stable in a couple of
months. Since then, be sure to run the latest nightly version with
`rustup update nightly`.

#### Binary

To compile the parser to a binary, run:

```sh
$ just build-binary
$ ./target/release/gutenberg-post-parser --emit-json tests/fixtures/gutenberg-demo.html
```

#### Static library

To compile the parser to a static library, run:

```sh
$ just build-library
$ ls target/release/
```

#### WebAssembly

To compile the parser to a [WebAssembly] binary, run:

```sh
$ just build-wasm
$ ./bindings/wasm/bin/gutenberg-post-parser --emit-json tests/fixtures/gutenberg-demo.html
```

If you would like to test directly in your browser, run:

```sh
$ just build-wasm
$ just start-wasm-server
$ open localhost:8888
```

Learn more about the [WebAssembly binding](./bindings/wasm/).

#### ASM.js

To compile the parser to an [ASM.js] module, run:

```sh
$ just build-asmjs
$ just start-asmjs-server
$ open localhost:8888
```

The ASM.js module is slower than the WebAssembly binary, but it is
useful for Internet Explorer compatibility, or any browser that does
not support WebAssembly. Remember that ASM.js is just a JavaScript
file.

Learn more about the [ASM.js binding](./bindings/asmjs/).

#### NodeJS

To compile the parser to a [NodeJS] native module, run:

```sh
$ just build-nodejs
$ ./bindings/nodejs/bin/gutenberg-post-parser --emit-json tests/fixtures/gutenberg-demo.html
```

Learn more about the [NodeJS binding](./bindings/nodejs/).

#### C

To compile the parser to a [C header][C], run:

```sh
$ just build-c
$ ./bindings/c/bin/gutenberg-post-parser tests/fixtures/gutenberg-demo.html
```

#### PHP

To compile the parser to a [PHP extension][PHP], run:

```sh
$ just build-php
$ ./bindings/php/bin/gutenberg-post-parser --emit-debug tests/fixtures/gutenberg-demo.html
```

To load the extension, add `extension=gutenberg_post_parser` in the
`php.ini` file (hint: Run `php --ini` to locate this configuration
file), or run PHP such as `php -d extension=gutenberg_post_parser
file.php`.

Learn more about the [PHP binding](./bindings/php/).

### Performance and guarantee

The parser guarantees to never copy the data in memory while
analyzing, which makes it fast and memory efficient.

#### WASM binary

[A yet-to-be-official benchmark][gutenberg-parser-comparator] is used
to compare the performance of the actual Javascript parser against the
Rust parser compiled as a WASM binary so that it can run in the
browser. Here are the results:

| file | Javascript parser (ms) | Rust parser as a WASM binary (ms) | speedup |
|-|-|-|-|
| [`demo-post.html`] | 13.167 | 0.43 | × 31 |
| [`shortcode-shortcomings.html`] | 26.784 | 0.476 | × 56 |
| [`redesigning-chrome-desktop.html`] | 75.500 | 1.811 | × 42 |
| [`web-at-maximum-fps.html`] | 88.118 | 1.334 | × 66 |
| [`early-adopting-the-future.html`] | 201.011 | 3.171 | × 63 |
| [`pygmalian-raw-html.html`] | 311.416 | 2.894 | × 108 |
| [`moby-dick-parsed.html`] | 2,466.533 | 23.62 | × 104 |

The WASM binary of the Rust parser is in average 67 times faster than
the actual Javascript implementation. The median of the speedup is 63.

#### PHP native extension

Another benchmark has been used to compare the performance of the
actual PHP parser against the Rust parser compiled as a PHP native
extension. Here are the results:

| file | PHP parser (ms) | Rust parser as a PHP native extension (ms) | speedup |
|-|-|-|-|
| [`demo-post.html`] | 14.656 | 0.019 | × 771 |
| [`shortcode-shortcomings.html`] | 38.624 | 0.022 | × 1756 |
| [`redesigning-chrome-desktop.html`] | 109.377 | 0.090 | × 1215 |
| [`web-at-maximum-fps.html`] | 87.345 | 0.061 | × 1432 |
| [`early-adopting-the-future.html`] | 137.056 | 0.074 | × 1852 |
| [`pygmalian-raw-html.html`] | 165.072 | 0.009 | × 18341 |
| [`moby-dick-parsed.html`] | 2,681.936 | 0.993 | × 2701 |

The PHP extension of the Rust parser is in average 4010 times faster
than the actual PHP implementation. The median of the speedup is 1756.

Note that memory limit has been hit very quickly with the PHP parser,
while the Rust parser as a PHP native extension has a small memory
footprint.

## License

The license is a classic `BSD-3-Clause`:

> New BSD License
>
> Copyright © Ivan Enderlin. All rights reserved.
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
[ASM.js]: http://asmjs.org/spec/latest/
[NodeJS]: https://nodejs.org/
[C]: https://en.wikipedia.org/wiki/C_(programming_language)
[PHP]: https://php.net/
[gutenberg-parser-comparator]: https://github.com/dmsnell/gutenberg-parser-comparator
[`demo-post.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/demo-post.html
[`shortcode-shortcomings.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/shortcode-shortcomings.html
[`redesigning-chrome-desktop.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/redesigning-chrome-desktop.html
[`web-at-maximum-fps.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/web-at-maximum-fps.html
[`early-adopting-the-future.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/early-adopting-the-future.html
[`pygmalian-raw-html.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/pygmalian-raw-html.html
[`moby-dick-parsed.html`]: https://raw.githubusercontent.com/dmsnell/gutenberg-document-library/master/library/moby-dick-parsed.html

*/


#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "no_std", feature(alloc))]

#[cfg(feature = "no_std")] #[macro_use] extern crate alloc;
#[macro_use] extern crate nom;

#[cfg(feature = "no_std")] use alloc::Vec;

// Export modules.
pub mod ast;
#[macro_use] pub mod combinators;
pub mod parser;


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
/// use gutenberg_post_parser::{root, ast::Node};
///
/// let input = &b"<!-- wp:foo {\"bar\": true} /-->"[..];
/// let output = Ok(
///     (
///         // The remaining data.
///         &b""[..],
///
///         // The Abstract Syntax Tree.
///         vec![
///             Node::Block {
///                 name: (&b"core"[..], &b"foo"[..]),
///                 attributes: Some(&b"{\"bar\": true}"[..]),
///                 children: vec![]
///             }
///         ]
///     )
/// );
///
/// assert_eq!(root(input), output);
/// ```
pub fn root(input: Input) -> Result<(Input, Vec<ast::Node>), nom::Err<Input>> {
    parser::block_list(input)
}
