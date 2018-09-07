# The NodeJS binding of the Gutenberg post parser

The NodeJS binding relies on the [Neon project][Neon].

## Installation

Please, see the [`README.md`](../../README.md) of the project, but
basically, it reduces to:

```sh
$ just build-nodejs
```

Usage example:

```js
const gutenberg_post_parser = require('bindings/nodejs/native');

console.log(
    gutenberg_post_parser.root('<!-- wp:foo /-->')
);
```

## Execute from NodeJS

The `./bin/gutenberg-post-parser` executable is a NodeJS script using
the NodeJS native module to parse a Gutenberg post and to emit either
JSON or debug data.

```sh
$ ./bin/gutenberg-post-parser --emit-json ../../tests/fixtures/autoclosing-block.html
```

[Neon]: https://www.neon-bindings.com/
