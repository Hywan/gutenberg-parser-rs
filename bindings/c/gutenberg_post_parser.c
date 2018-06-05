#include <stdio.h>
#include <stdlib.h>
#include "gutenberg_post_parser.h"

int main() {
  const char input[] = "<!-- wp:foo {\"abc\":true} /-->bar<!-- wp:baz -->qux<!-- /wp:baz -->";

  printf("%s\n", input);

  Result output = parse(input);

  if (output.tag == Err) {
    printf("Parse error\n");

    return 1;
  }

  const uintptr_t number_of_nodes = output.ok._0.length;

  for (uintptr_t nth = 0; nth < number_of_nodes; ++nth) {
    const Node node = output.ok._0.buffer[nth];

    if (node.tag == Block) {
      const Block_Body block = node.block;

      printf("block\n");
      printf("  %s/%s\n", block.namespace, block.name);

      if (block.attributes.tag == Some) {
        const char *attributes = block.attributes.some._0;

        printf("  %s\n", attributes);
      }

      printf(" ~~~> block.children = %p\n", block.children);
      const Vector_Node* children = (const Vector_Node*) (block.children);
      printf(" ===> %p\n", children);
      printf(" ===> %lu\n", children->length);
    } else {
      const char *phrase = node.phrase._0;

      printf("phrase\n");
      printf("  %s\n", phrase);
    }

    printf("\n");
  }

  printf("%lu\n", number_of_nodes);

  return 0;
}
