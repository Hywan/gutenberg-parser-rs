## The Gutenberg post parser.

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

### Platforms and bindings

The parser aims at being used on different platforms, such as: Web
within multiple browsers, Web applications like [Electron], native
applications like macOS, iOS, Windows, Linux etc.

Thus, the parser can be compiled as a static library, can be embedded
in any Rust projects, can be compiled to [WebAssembly], and soon more.

This project uses [Justfile] as an alternative to Makefile. Every
following command will use `just`, you might consider to install
it. To learn about all the commands, just `just --list`.

#### Static library

To compile the parser to a static library, run:

```sh
$ just build-library
$ ls target/release/
```

#### WebAssembly

To compile the parser to a [WebAssembly] file, run:

```sh
$ just build-wasm
$ open bindings/wasm/index.html # for a demonstration
```

### Performance and guarantee

The parser guarantees to never copy the data in memory, which makes it
fast and memory efficient.

### License

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

