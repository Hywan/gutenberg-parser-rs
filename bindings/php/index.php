<?php

const POST = '<!-- wp:foo {"abc": "def"} /-->bar<!-- wp:b/az -->qux<!-- wp:hello /--><!-- /wp:b/az -->';

var_dump(gutenberg_post_parse(POST));
