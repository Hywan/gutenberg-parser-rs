#include <stdio.h>
#include <stdlib.h>
#include "gutenberg_post_parser.h"

int main() {
  char input[] = "<!-- wp:foo {\"abc\":true} /-->bar<!-- wp:baz -->qux<!-- /wp:baz -->";
  Result output = parse(input);

  printf("%s\n", input);

  if (output.tag == Err) {
    printf("Parse error\n");

    return 1;
  }

  uintptr_t number_of_nodes = output.ok._0.length;

  for (uintptr_t nth = 0; nth < number_of_nodes; ++nth) {
    Node node = output.ok._0.buffer[nth];

    if (node.tag == Block) {
      Block_Body block = node.block;

      printf("block\n");
      printf("  %s/%s\n", block.namespace, block.name);

      if (block.attributes.tag == Some) {
        const char *attributes = block.attributes.some._0;

        printf("  %s\n", attributes);
      }

      Vector_Node* children = (Vector_Node*) block.children;
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
