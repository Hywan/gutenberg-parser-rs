# The PHP binding of the Gutenberg post parser

The [PHP] binding of the Gutenberg post parser is designed as follows:

  * The Gutenberg post parser is a Rust project compiled as a [static
    library] + C headers (see the [C binding](../c/)),
  * A PHP extension is created and linked to the static library and
    the C headers, thus exposing the Rust Gutenberg parser API into
    PHP.

## Installation

Please, see the [`README.md`](../../README.md) of the project, but
basically, it reduces to:

```sh
$ just build-php
```

It is possible to build the PHP binding for a specific PHP version by running:

```sh
$ just build-php '/path/to/php-bins'
```

The `/path/to/php-bins` directory must contain a `phpize`
executable. It is essential to compile the extension correctly against
the desired PHP version.

A copy of the extension is automatically installed. The original extension file is in `./extension/gutenberg_post_parser/modules/`.

To enable the extension, run `php` with the option `-d extension=gutenberg_post_parser`. To permanently load the extension, locate the `php.ini` file with `php --ini`, and edit it to add:

```ini
extension=gutenberg_post_parser
```

Then, enjoy:

```sh
$ # Extension is loaded as a module
$ php -d extension=gutenberg_post_parser -m | grep gutenberg_post_parser

$ # Extension is correctly defined
$ php -d extension=gutenberg_post_parser --re gutenberg_post_parser

$ # Classes exist
$ php -d extension=gutenberg_post_parser --rc Gutenberg_Parser_Block
$ php -d extension=gutenberg_post_parser --rc Gutenberg_Parser_Phrase

$ # Function exists
$ php -d extension=gutenberg_post_parser --rf gutenberg_post_parse
```

Usage example:

```php
var_dump(
    gutenberg_post_parse('<!-- wp:foo /-->') // a collection containing one item: A `Gutenberg_Parser_Block` instance.
);
```

## Execute from PHP

The `./bin/gutenberg-post-parser` executable is a PHP program using
the PHP extension to parse a Gutenberg post and to emit either JSON or
debug data.

```sh
$ ./bin/gutenberg-post-parser --emit-debug ../../tests/fixtures/autoclosing-block.html
```

[PHP]: https://php.net/
[static library]: https://en.wikipedia.org/wiki/Static_library
