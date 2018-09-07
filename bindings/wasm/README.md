# The WebAssembly binding of the Gutenberg post parser

The [WebAssembly] binding of the Gutenberg post parser is designed as follows:

  * The Gutenberg post parser is a Rust project compiled to the
    `wasm32-unknown-unknown` target, resulting of a WASM binary file:
    `./bin/gutenberg_post_parser.wasm`,
  * The WASM binary is optimized to reduce its size with [`wasm-gc`],
    [`wasm-snip`], and [`wasm-opt`],
  * A boundary layer written in JavaScript is responsible of reading
    from and writing into the WASM memory, and exposing a JavaScript
    friendly API: `./bin/gutenberg_post_parser.mjs`. It is written as
    an [ECMAScript module][mjs], so that it can be used with an HTML
    document (with `<script type="module" src="…" />`) or in a NodeJS
    script (with the `--experimental-modules` option so far).

## Installation

Please, see the [`README.md`](../../README.md) of the project, but
basically, it reduces to:

```sh
$ just build-wasm
```

## The boundary layer

The boundary layer is the most important file after the WASM
binary. It exposes a user-friendly API to the JavaScript land. It
consists of a single class named `Gutenberg_Post_Parser`. Its
constructor expects a class to represent a `Block`, a class to
represent a `Phrase`, a WASM URL, a class for text encoding (by
default [`TextEncoder`]), and another one for text decoding (by
default [`TextDecoder`]).

Usage example:

``` js
import { Gutenberg_Post_Parser } from './gutenberg_post_parser.mjs';

class Block {
    constructor(name, attributes, children) {
        this.name = name;             // a string containing the block namespace and name.
        this.attributes = attributes; // a JSON object.
        this.children = children;     // a collection of `Block` and `Phrase` instances.
    }
}

class Phrase {
    constructor(phrase) {
        this.phrase = phrase; // a string.
    }
}

const parser = new Gutenberg_Post_Parser(
    Block,
    Phrase,
    './gutenberg_post_parser.wasm'
);

parser.root('<!-- wp:foo /-->').then(
    (output) => {
        console.log(output); // a collection containg a single item: A `Block` instance.
    }
);
```

The `Gutenberg_Post_Parser.instantiateWASM` method is responsible to
load and instantiate the WASM binary. To load it, by default, it uses
the [`Fetch` API]. In some cases, this API can be absent (like in
NodeJS). It is therefore recommended to override this method. The rest
of the code is just about manipulating the memory.

## Execute from a browser

The `./web/` directory contains:

  * An `index.html` document showing a demo,
  * An `index.mjs` file using the WASM binary and the boundary layer
    to parse the demo Gutenberg post and emit its JSON representation,
  * A `server.php` file to serve the WASM binary with the correct MIME
    type (when using [`WebAssembly.instantiateStreaming`], the WASM binary
    needs to be served with `application/wasm`).

```sh
$ just start-wasm-server
$ open localhost:8888
```

## Execute from NodeJS

The `./bin/gutenberg-post-parser` executable is a NodeJS script using
the WASM binary and the boundary layer to parse a Gutenberg post and
to emit either JSON or debug data.

```sh
$ ./bin/gutenberg-post-parser --emit-json ../../tests/fixtures/autoclosing-block.html
```

[WebAssembly]: http://webassembly.org/
[`wasm-gc`]: https://github.com/alexcrichton/wasm-gc
[`wasm-snip`]: https://github.com/fitzgen/wasm-snip
[`wasm-opt`]: https://github.com/WebAssembly/binaryen
[mjs]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import
[`WebAssembly.instantiateStreaming`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiateStreaming
[`TextEncoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder
[`TextDecoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder
[`Fetch` API]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
