/* gutenberg_post_parser extension for PHP (c) 2018 Ivan Enderlin */

#ifndef PHP_GUTENBERG_POST_PARSER_H
# define PHP_GUTENBERG_POST_PARSER_H

extern zend_module_entry gutenberg_post_parser_module_entry;
# define phpext_gutenberg_post_parser_ptr &gutenberg_post_parser_module_entry

# define PHP_GUTENBERG_POST_PARSER_VERSION "0.1.0"

# if defined(ZTS) && defined(COMPILE_DL_GUTENBERG_POST_PARSER)
ZEND_TSRMLS_CACHE_EXTERN()
# endif

#endif	/* PHP_GUTENBERG_POST_PARSER_H */
