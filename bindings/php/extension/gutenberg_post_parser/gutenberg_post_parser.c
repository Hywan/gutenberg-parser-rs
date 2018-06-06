/* gutenberg_post_parser extension for PHP (c) 2018 Ivan Enderlin */

#ifdef HAVE_CONFIG_H
# include "config.h"
#endif

#include "php.h"
#include "ext/standard/info.h"
#include "php_gutenberg_post_parser.h"
#include "gutenberg_post_parser.h"

/* {{{ string gutenberg_post_parser( [ string $var ] )
 */
PHP_FUNCTION(gutenberg_post_parser)
{
	char *input;
	size_t input_len;

	if (zend_parse_parameters(ZEND_NUM_ARGS() TSRMLS_CC, "s", &input, &input_len) == FAILURE) {
		return;
	}

	Result parser_result = parse(input);

	if (parser_result.tag == Err) {
		RETURN_FALSE;
	}

	const Vector_Node nodes = parser_result.ok._0;

	RETURN_LONG((int) (nodes.length));
}
/* }}}*/

/* {{{ PHP_MINIT_FUNCTION
 */
zend_class_entry *global_gutenberg_parser_block_class_entry;
zend_class_entry *global_gutenberg_parser_phrase_class_entry;

const zend_function_entry gutenberg_parser_functions[] = {
	PHP_FE_END
};

PHP_MINIT_FUNCTION(gutenberg_post_parser)
{
	zend_class_entry gutenberg_parser_block_class_entry;
	INIT_CLASS_ENTRY(gutenberg_parser_block_class_entry, "Gutenberg_Parser_Block", gutenberg_parser_functions);

	zend_class_entry gutenberg_parser_phrase_class_entry;
	INIT_CLASS_ENTRY(gutenberg_parser_phrase_class_entry, "Gutenberg_Parser_Phrase", gutenberg_parser_functions);

	global_gutenberg_parser_block_class_entry = zend_register_internal_class(&gutenberg_parser_block_class_entry TSRMLS_CC);
	global_gutenberg_parser_phrase_class_entry = zend_register_internal_class(&gutenberg_parser_phrase_class_entry TSRMLS_CC);


	return SUCCESS;
}
/* }}} */

/* {{{ PHP_RINIT_FUNCTION
 */
PHP_RINIT_FUNCTION(gutenberg_post_parser)
{
#if defined(ZTS) && defined(COMPILE_DL_GUTENBERG_POST_PARSER)
	ZEND_TSRMLS_CACHE_UPDATE();
#endif

	return SUCCESS;
}
/* }}} */

/* {{{ PHP_MINFO_FUNCTION
 */
PHP_MINFO_FUNCTION(gutenberg_post_parser)
{
	php_info_print_table_start();
	php_info_print_table_header(2, "gutenberg_post_parser support", "enabled");
	php_info_print_table_end();
}
/* }}} */

/* {{{ arginfo
 */
ZEND_BEGIN_ARG_INFO(arginfo_gutenberg_post_parser, 0)
	ZEND_ARG_INFO(0, who)
ZEND_END_ARG_INFO()
/* }}} */

/* {{{ gutenberg_post_parser_functions[]
 */
static const zend_function_entry gutenberg_post_parser_functions[] = {
	PHP_FE(gutenberg_post_parser,		arginfo_gutenberg_post_parser)
	PHP_FE_END
};
/* }}} */

/* {{{ gutenberg_post_parser_module_entry
 */
zend_module_entry gutenberg_post_parser_module_entry = {
	STANDARD_MODULE_HEADER,
	"gutenberg_post_parser",					/* Extension name */
	gutenberg_post_parser_functions,			/* zend_function_entry */
	PHP_MINIT(gutenberg_post_parser),			/* PHP_MINIT - Module initialization */
	NULL,							/* PHP_MSHUTDOWN - Module shutdown */
	PHP_RINIT(gutenberg_post_parser),			/* PHP_RINIT - Request initialization */
	NULL,							/* PHP_RSHUTDOWN - Request shutdown */
	PHP_MINFO(gutenberg_post_parser),			/* PHP_MINFO - Module info */
	PHP_GUTENBERG_POST_PARSER_VERSION,		/* Version */
	STANDARD_MODULE_PROPERTIES
};
/* }}} */

#ifdef COMPILE_DL_GUTENBERG_POST_PARSER
# ifdef ZTS
ZEND_TSRMLS_CACHE_DEFINE()
# endif
ZEND_GET_MODULE(gutenberg_post_parser)
#endif
