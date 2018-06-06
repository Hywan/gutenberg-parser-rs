#!/usr/bin/env php -dextension=gutenberg_post_parser
<?php

if (false === extension_loaded('gutenberg_post_parser')) {
    die('The `gutenberg_post_parser` extension is not loaded.');
}

const POST = '<!-- wp:foo {"abc": "def"} /-->bar<!-- wp:b/az -->qux<!-- wp:hello /--><!-- /wp:b/az -->';

var_dump(gutenberg_post_parse(POST));
