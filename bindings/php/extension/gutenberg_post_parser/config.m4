PHP_ARG_ENABLE(gutenberg_post_parser, whether to enable gutenberg_post_parser support,
[  --with-gutenberg_post_parser          Include gutenberg_post_parser support], no)

if test "$PHP_GUTENBERG_POST_PARSER" != "no"; then
  PHP_SUBST(GUTENBERG_POST_PARSER_SHARED_LIBADD)

  PHP_ADD_LIBRARY_WITH_PATH(gutenberg_post_parser, ., GUTENBERG_POST_PARSER_SHARED_LIBADD)

  PHP_NEW_EXTENSION(gutenberg_post_parser, gutenberg_post_parser.c, $ext_shared)
fi
