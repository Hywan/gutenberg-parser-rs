/* gutenberg_post_parser extension for PHP (c) 2018 Ivan Enderlin */

#ifdef HAVE_CONFIG_H
# include "config.h"
#endif

#include "php.h"
#include "ext/standard/info.h"
#include "php_gutenberg_post_parser.h"
#include "gutenberg_post_parser.h"

/* {{{ PHP_MINIT_FUNCTION
 */

// Class entry for `Gutenberg_Parser_Block` and `Gutenberg_Parser_Phrase`.
zend_class_entry *gutenberg_parser_block_class_entry;
zend_class_entry *gutenberg_parser_phrase_class_entry;

// Methods for `Gutenberg_Parser_*` classes. There is no method.
const zend_function_entry gutenberg_post_parser_methods[] = {
	PHP_FE_END
};

// Initialize the module.
PHP_MINIT_FUNCTION(gutenberg_post_parser)
{
	zend_class_entry class_entry;

	// Declare the `Gutenberg_Parser_Block` class.
	INIT_CLASS_ENTRY(class_entry, "Gutenberg_Parser_Block", gutenberg_post_parser_methods);
	gutenberg_parser_block_class_entry = zend_register_internal_class(&class_entry TSRMLS_CC);

	// The class is final.
	gutenberg_parser_block_class_entry->ce_flags |= ZEND_ACC_FINAL;

	// Declare the `namespace` public attribute, with an empty string for the default value.
	zend_declare_property_string(gutenberg_parser_block_class_entry, "namespace", sizeof("namespace") - 1, "", ZEND_ACC_PUBLIC);

	// Declare the `name` public attribute, with an empty string for the default value.
	zend_declare_property_string(gutenberg_parser_block_class_entry, "name", sizeof("name") - 1, "", ZEND_ACC_PUBLIC);
	
	// Declare the `attributes` public attribute, with NULL for the default value.
	zend_declare_property_null(gutenberg_parser_block_class_entry, "attributes", sizeof("attributes") - 1, ZEND_ACC_PUBLIC);


	// Declare the `Gutenberg_Parser_Phrase` class.
	INIT_CLASS_ENTRY(class_entry, "Gutenberg_Parser_Phrase", gutenberg_post_parser_methods);
	gutenberg_parser_phrase_class_entry = zend_register_internal_class(&class_entry TSRMLS_CC);

	// The class is final.
	gutenberg_parser_phrase_class_entry->ce_flags |= ZEND_ACC_FINAL;

	// Declare the `content` public attribute, with an empty string for the default value.
	zend_declare_property_string(gutenberg_parser_block_class_entry, "name", sizeof("name") - 1, "", ZEND_ACC_PUBLIC);

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

void print(const Vector_Node* nodes, int depth)
{
	const uintptr_t number_of_nodes = nodes->length;
	
	if (number_of_nodes == 0) {
		return;
	}

	printf("%*.*snumber of nodes = %lu\n\n", depth, depth, " ", number_of_nodes);

	for (uintptr_t nth = 0; nth < number_of_nodes; ++nth) {
		const Node node = nodes->buffer[nth];

		if (node.tag == Block) {
			const Block_Body block = node.block;
			const char *namespace = block.namespace;
			const char *name = block.name;

			printf("%*.*sblock\n", depth, depth, " ");

			printf("%*.*s    %s/%s\n", depth, depth, " ", namespace, name);

			if (block.attributes.tag == Some) {
				const char *attributes = block.attributes.some._0;

				printf("%*.*s    %s\n", depth, depth, " ", attributes);
			}

			const Vector_Node* children = (const Vector_Node*) (block.children);

			print(children, depth + 4);
		} else if (node.tag == Phrase) {
			const char *phrase = node.phrase._0;

			printf("%*.*sphrase\n", depth, depth, " ");
			printf("%*.*s    %s\n", depth, depth, " ", phrase);
		}

		printf("\n");
	}
}

void into_php_objects(zval *array, const Vector_Node* nodes)
{
	const uintptr_t number_of_nodes = nodes->length;

	if (number_of_nodes == 0) {
		return;
	}

	for (uintptr_t nth = 0; nth < number_of_nodes; ++nth) {
		const Node node = nodes->buffer[nth];

		if (node.tag == Block) {
			const Block_Body block = node.block;
			zval php_block;

			object_init_ex(&php_block, gutenberg_parser_block_class_entry);
			add_property_string(&php_block, "namespace", block.namespace);
			add_property_string(&php_block, "name", block.name);

			if (block.attributes.tag == Some) {
				const char *attributes = block.attributes.some._0;

				add_property_string(&php_block, "attributes", attributes);
			}

			add_next_index_zval(array, &php_block);
		} else if (node.tag == Phrase) {
			const char *phrase = node.phrase._0;
			zval php_phrase;

			object_init_ex(&php_phrase, gutenberg_parser_phrase_class_entry);
			add_property_string(&php_phrase, "content", phrase);

			add_next_index_zval(array, &php_phrase);
		}
	}

	/*
	zval obj;
	object_init_ex(&obj, gutenberg_parser_block_class_entry);
	add_property_string(&obj, "name", "foobar");
	
	add_next_index_long(array, 101);
	add_next_index_zval(array, &obj);
	*/
}

/* {{{ string gutenberg_post_parser( [ string $var ] )
 */
PHP_FUNCTION(gutenberg_post_parse)
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

	array_init(return_value);
	into_php_objects(return_value, &nodes);
}
/* }}}*/

/* {{{ arginfo
 */
ZEND_BEGIN_ARG_INFO(arginfo_gutenberg_post_parser, 0)
	ZEND_ARG_INFO(0, gutenberg_post_as_string)
ZEND_END_ARG_INFO()
/* }}} */

/* {{{ gutenberg_post_parser_functions[]
 */
static const zend_function_entry gutenberg_post_parser_functions[] = {
	PHP_FE(gutenberg_post_parse,		arginfo_gutenberg_post_parser)
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

/*
 * Local variables:
 * tab-width: 4
 * c-basic-offset: 4
 * indent-tabs-mode: t
 * End:
 */
