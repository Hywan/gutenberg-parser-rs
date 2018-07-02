# The ASM.js binding of the Gutenberg post parser

The [ASM.js] binding of the Gutenberg post parser is a fallback for
the [WebAssembly binding](../wasm/) when the platform does not
it. ASM.js is a subset of JavaScript, so even if the platform does not
optimize this specific subset, the program is executed as a regular
JavaScript program.

The ASM.js binding is designed as follow:

  * The Gutenberg post parser is a Rust project compiled to the
    `wasm32-unknown-unknown` target, and the resulting WASM binary
    file is optimised,
  * Then, [`wasm2es6js`] is used to compiled the WASM binary as a
    ASM.js file. Internally, it uses the `wasm-dis` and the `wasm2asm`
    commands from the Binaryen project. This project is already
    required by the WebAssembly binding, so they are supposed to be
    installed,
  * The WebAssembly boundary layer is extended to be adapted to
    ASM.js, but the same properties hold: Reading from and writing
    into the ASM.js memory, and exposing a JavaScript friendly API.

## Installation

Please, see the [`README.md`](../../README.md) of the project, but
basically, it reduces to:

```sh
$ just build-asmjs
```

## The boundary layer

The boundary layer of the ASM.js binding extends the one from the
WebAssembly binding.

Usage example. The `./bin/gutenberg_post_Parser.asm.js` file must be
included before calling this one. Thus:

``` js
import { Gutenberg_Post_Parser_ASM } from './gutenberg_post_parser.asm.mjs';

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

const parser = new Gutenberg_Post_Parser_ASM(
    Block,
    Phrase,
    GUTENBERG_POST_PARSER_ASM_MODULE()
);

console.log(
    parser.root('<!-- wp:foo /-->') // a collection containg a single item: A `Block` instance.
);
```

It is possible that the [`TextEncoder`] and [`TextDecoder`] classes
are not present in Internet Explorer for instance. There is plenty of
polyfill out there. Those API are required.

## Execute from a browser

The `./web/` directory contains:

  * An `index.html` document showing a demo,
  * An `index.mjs` file using the ASM.js file and the boundary layer
    to parse the demo Gutenberg post and emit its JSON representation,
  * A `server.php` file to serve the demo.

```sh
$ just start-asmjs-server
$ open localhost:8888
```

[ASM.js]: http://asmjs.org/spec/latest/
[`wasm2es6js`]: https://github.com/rustwasm/wasm-bindgen/
[`Fetch` API]: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
[`TextEncoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder
[`TextDecoder`]: https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder
