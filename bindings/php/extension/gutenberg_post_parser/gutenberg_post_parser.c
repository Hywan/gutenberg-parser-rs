/* gutenberg_post_parser extension for PHP (c) 2018 Ivan Enderlin */

#ifdef HAVE_CONFIG_H
# include "config.h"
#endif

#include "php.h"
#include "ext/standard/info.h"
#include "php_gutenberg_post_parser.h"
#include "gutenberg_post_parser.h"

/*
 * Class entry for `Gutenberg_Parser_Block` and `Gutenberg_Parser_Phrase`.
 */
zend_class_entry *gutenberg_parser_block_class_entry;
zend_class_entry *gutenberg_parser_phrase_class_entry;
zend_object_handlers gutenberg_parser_node_class_entry_handlers;

/*
 * Custom object for Gutenberg parser nodes.
 */
typedef struct _gutenberg_parser_block {
	zend_object zobj;
} gutenberg_parser_node;


/*
 * Function for a `zend_class_entry` to create a Gutenberg parser node object.
 */
static zend_object *create_parser_node_object(zend_class_entry *class_entry)
{
	gutenberg_parser_node *gutenberg_parser_node_object;

	gutenberg_parser_node_object = ecalloc(1, sizeof(*gutenberg_parser_node_object) + zend_object_properties_size(class_entry));

	zend_object_std_init(&gutenberg_parser_node_object->zobj, class_entry);
	object_properties_init(&gutenberg_parser_node_object->zobj, class_entry);

	gutenberg_parser_node_object->zobj.handlers = &gutenberg_parser_node_class_entry_handlers;

	return &gutenberg_parser_node_object->zobj;
}

/*
 * Handler for a `zend_class_entry` to destroy (i.e. call the
 * destructor on the userland for) a Gutenberg parser node object.
 */
static void destroy_parser_node_object(zend_object *gutenberg_parser_node_object)
{
	zend_objects_destroy_object(gutenberg_parser_node_object);
}

/*
 * Handler for a `zend_class_entry` to free a Gutenberg parser node object.
 */
static void free_parser_node_object(zend_object *gutenberg_parser_node_object)
{
	zend_object_std_dtor(gutenberg_parser_node_object);
}

/*
 * Initialize the module.
 */
PHP_MINIT_FUNCTION(gutenberg_post_parser)
{
	zend_class_entry class_entry;

	//
	// Declare `Gutenberg_Parser_Block`.
	//

	INIT_CLASS_ENTRY(class_entry, "Gutenberg_Parser_Block", NULL);
	gutenberg_parser_block_class_entry = zend_register_internal_class(&class_entry TSRMLS_CC);

	// Declare the create handler.
	gutenberg_parser_block_class_entry->create_object = create_parser_node_object;

	// The class is final.
	gutenberg_parser_block_class_entry->ce_flags |= ZEND_ACC_FINAL;

	// Declare the `namespace` public attribute, with an empty string for the default value.
	zend_declare_property_string(gutenberg_parser_block_class_entry, "namespace", sizeof("namespace") - 1, "", ZEND_ACC_PUBLIC);

	// Declare the `name` public attribute, with an empty string for the default value.
	zend_declare_property_string(gutenberg_parser_block_class_entry, "name", sizeof("name") - 1, "", ZEND_ACC_PUBLIC);
	
	// Declare the `attributes` public attribute, with `NULL` for the default value.
	zend_declare_property_null(gutenberg_parser_block_class_entry, "attributes", sizeof("attributes") - 1, ZEND_ACC_PUBLIC);
	
	// Declare the `children` public attribute, with `NULL` for the default value.
	zend_declare_property_null(gutenberg_parser_block_class_entry, "children", sizeof("children") - 1, ZEND_ACC_PUBLIC);

	//
	// Declare `Gutenberg_Parser_Phrase`.
	//
	
	INIT_CLASS_ENTRY(class_entry, "Gutenberg_Parser_Phrase", NULL);
	gutenberg_parser_phrase_class_entry = zend_register_internal_class(&class_entry TSRMLS_CC);

	// Declare the create handler.
	gutenberg_parser_phrase_class_entry->create_object = create_parser_node_object;

	// The class is final.
	gutenberg_parser_phrase_class_entry->ce_flags |= ZEND_ACC_FINAL;

	// Declare the `content` public attribute, with an empty string for the default value.
	zend_declare_property_string(gutenberg_parser_phrase_class_entry, "content", sizeof("content") - 1, "", ZEND_ACC_PUBLIC);

	//
	// Declare Gutenberg parser node object handlers.
	//

	memcpy(&gutenberg_parser_node_class_entry_handlers, zend_get_std_object_handlers(), sizeof(gutenberg_parser_node_class_entry_handlers));

	gutenberg_parser_node_class_entry_handlers.offset = XtOffsetOf(gutenberg_parser_node, zobj);
	gutenberg_parser_node_class_entry_handlers.dtor_obj = destroy_parser_node_object;
	gutenberg_parser_node_class_entry_handlers.free_obj = free_parser_node_object;

	return SUCCESS;
}

PHP_RINIT_FUNCTION(gutenberg_post_parser)
{
#if defined(ZTS) && defined(COMPILE_DL_GUTENBERG_POST_PARSER)
	ZEND_TSRMLS_CACHE_UPDATE();
#endif

	return SUCCESS;
}

/*
 * Provide information about the module.
 */
PHP_MINFO_FUNCTION(gutenberg_post_parser)
{
	php_info_print_table_start();
	php_info_print_table_header(2, "gutenberg_post_parser support", "enabled");
	php_info_print_table_end();
}

/*
 * Map Rust AST to a PHP array of objects of kind
 * `Gutenberg_Parser_Block` and `Gutenberg_Parser_Phrase`.
 */
void into_php_objects(zval *php_array, const Vector_Node* nodes)
{
	const uintptr_t number_of_nodes = nodes->length;

	if (number_of_nodes == 0) {
		free((void*) nodes->buffer);

		return;
	}

	// Iterate over all nodes.
	for (uintptr_t nth = 0; nth < number_of_nodes; ++nth) {
		const Node node = nodes->buffer[nth];

		// Map [rust] `Node::Block` to [php] `Gutenberg_Parser_Block`.
		if (node.tag == Block) {
			const Block_Body block = node.block;
			zval php_block, php_block_namespace, php_block_name;

			// Prepare the PHP strings.
			ZVAL_STRINGL(&php_block_namespace, block.namespace.pointer, block.namespace.length);
			ZVAL_STRINGL(&php_block_name, block.name.pointer, block.name.length);

			// Create the `Gutenberg_Parser_Block` object.
			object_init_ex(&php_block, gutenberg_parser_block_class_entry);

			// Map [rust] `Node::Block.name.0` to [php] `Gutenberg_Parser_Block->namespace`.
			add_property_zval(&php_block, "namespace", &php_block_namespace);

			// Map [rust] `Node::Block.name.1` to [php] `Gutenberg_Parser_Block->name`.
			add_property_zval(&php_block, "name", &php_block_name);

			// Writing the property adds 1 to refcount.
			zval_ptr_dtor(&php_block_namespace);
			zval_ptr_dtor(&php_block_name);

			// Default value for `Gutenberg_Parser_Block->attributes` is `NULL`.
			// Allocate a string only if some value.
			if (block.attributes.tag == Some) {
				Slice_c_char attributes = block.attributes.some._0;

				zval php_block_attributes;

				// Prepare the PHP string.
				ZVAL_STRINGL(&php_block_attributes, attributes.pointer, attributes.length);

				// Map [rust] `Node::Block.attributes` to [php] `Gutenberg_Parser_Block->attributes`.
				add_property_zval(&php_block, "attributes", &php_block_attributes);

				// Writing the property adds 1 to refcount.
				zval_ptr_dtor(&php_block_attributes);
			}

			const Vector_Node* children = (const Vector_Node*) (block.children);

			// Default value for `Gutenberg_Parser_Block->children` is `NULL`.
			// Allocate an array only if there is children.
			if (children->length > 0) {
				zval php_children_array;

				array_init_size(&php_children_array, children->length);
				into_php_objects(&php_children_array, children);

				// Map [rust] `Node::Block.children` to [php] `Gutenberg_Parser_Block->children`.
				add_property_zval(&php_block, "children", &php_children_array);

				Z_DELREF(php_children_array);
			}

			free((void*) children);

			// Insert `Gutenberg_Parser_Block` into the collection.
			add_next_index_zval(php_array, &php_block);
		}
		// Map [rust] `Node::Phrase` to [php] `Gutenberg_Parser_Phrase`.
		else if (node.tag == Phrase) {
			Slice_c_char phrase = node.phrase._0;

			zval php_phrase, php_phrase_content;

			// Prepare the PHP string.
			ZVAL_STRINGL(&php_phrase_content, phrase.pointer, phrase.length);

			// Create the `Gutenberg_Parser_Phrase` object.
			object_init_ex(&php_phrase, gutenberg_parser_phrase_class_entry);

			// Map [rust] `Node::Phrase(content)` to [php] `Gutenberg_Parser_Phrase->content`.
			add_property_zval(&php_phrase, "content", &php_phrase_content);

			// Insert `Gutenberg_Parser_Phrase` into the collection.
			add_next_index_zval(php_array, &php_phrase);

			// Writing the property adds 1 to refcount.
			zval_ptr_dtor(&php_phrase_content);
		}
	}

	free((void*) nodes->buffer);
}

/*
 * Declare the `gutenberg_post_parse(string $gutenberg_post_as_string): bool | array;` function.
 */
PHP_FUNCTION(gutenberg_post_parse)
{
	char *input;
	size_t input_len;

	// Read the input as a string.
	if (zend_parse_parameters(ZEND_NUM_ARGS() TSRMLS_CC, "s", &input, &input_len) == FAILURE) {
		return;
	}

	// Parse the input.
	Result parser_result = parse(input);

	// If parsing failed, then return `false`.
	if (parser_result.tag == Err) {
		RETURN_FALSE;
	}

	// Else map the Rust AST into a PHP array.
	const Vector_Node nodes = parser_result.ok._0;

	// Note: `return_value` is a “magic” variable that holds the value to be returned.
	//
	// Allocate an array.
	array_init_size(return_value, nodes.length);

	// Map the Rust AST.
	into_php_objects(return_value, &nodes);
}

/*
 * Provide arginfo.
 */
ZEND_BEGIN_ARG_INFO(arginfo_gutenberg_post_parser, 0)
	ZEND_ARG_INFO(0, gutenberg_post_as_string)
ZEND_END_ARG_INFO()

/*
 * Declare functions.
 */
static const zend_function_entry gutenberg_post_parser_functions[] = {
	PHP_FE(gutenberg_post_parse,		arginfo_gutenberg_post_parser)
	PHP_FE_END
};

/*
 * Declare the module.
 */
zend_module_entry gutenberg_post_parser_module_entry = {
	STANDARD_MODULE_HEADER,
	"gutenberg_post_parser",					/* Extension name */
	gutenberg_post_parser_functions,			/* zend_function_entry */
	PHP_MINIT(gutenberg_post_parser),			/* PHP_MINIT - Module initialization */
	NULL,										/* PHP_MSHUTDOWN - Module shutdown */
	PHP_RINIT(gutenberg_post_parser),			/* PHP_RINIT - Request initialization */
	NULL,										/* PHP_RSHUTDOWN - Request shutdown */
	PHP_MINFO(gutenberg_post_parser),			/* PHP_MINFO - Module info */
	PHP_GUTENBERG_POST_PARSER_VERSION,			/* Version */
	STANDARD_MODULE_PROPERTIES
};

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
