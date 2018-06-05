#include <stdio.h>
#include <stdlib.h>
#include "gutenberg_post_parser.h"

void print(const Vector_Node* nodes, int depth) {
  const uintptr_t number_of_nodes = nodes->length;

  if (number_of_nodes == 0) {
    return;
  }

  printf("%*.*snumber of nodes = %lu\n\n", depth, depth, " ", number_of_nodes);

  for (uintptr_t nth = 0; nth < number_of_nodes; ++nth) {
    const Node node = nodes->buffer[nth];

    if (node.tag == Block) {
      const Block_Body block = node.block;

      printf("%*.*sblock\n", depth, depth, " ");
      printf("%*.*s    %s/%s\n", depth, depth, " ", block.namespace, block.name);

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

int main() {
  const char input[] = "<!-- wp:foo {\"abc\":true} /-->bar<!-- wp:baz --><!-- wp:qu/xxx /-->xyz<!-- /wp:baz -->";

  printf("%s\n\n", input);

  Result output = parse(input);

  if (output.tag == Err) {
    printf("Parse error\n");

    return 1;
  }

  const Vector_Node nodes = output.ok._0;
  print(&nodes, 0);

  return 0;
}
