#include <stdio.h>
#include <stdlib.h>
#include <string.h>
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
            const Slice_c_char namespace = block.namespace;
            const Slice_c_char name = block.name;

            printf("%*.*sblock\n", depth, depth, " ");
            printf(
                "%*.*s    %.*s/%.*s\n",
                depth, depth, " ",
                (int) namespace.length, namespace.pointer,
                (int) name.length, name.pointer
            );

            if (block.attributes.tag == Some) {
                const Slice_c_char attributes = block.attributes.some._0;

                printf(
                    "%*.*s    %.*s\n",
                    depth, depth, " ",
                    (int) attributes.length, attributes.pointer
                );
            }

            const Vector_Node* children = (const Vector_Node*) (block.children);

            print(children, depth + 4);

            free((void*) children);
        } else if (node.tag == Phrase) {
            const Slice_c_char phrase = node.phrase._0;

            printf("%*.*sphrase\n", depth, depth, " ");
            printf(
                "%*.*s    %.*s\n",
                depth, depth, " ",
                (int) phrase.length, phrase.pointer
            );
        }

        printf("\n");
    }

    free((void*) nodes->buffer);
}

int main(int argc, char **argv) {
    if (argc < 2) {
        printf("Filename is required.");

        return 1;
    }

    FILE* file = fopen(argv[1], "rb");

    if (file == NULL) {
        printf("Cannot read given file `%s`.", argv[1]);

        return 2;
    }

    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    rewind(file);

    char* file_content = (char*) malloc(file_size * sizeof(char));
    size_t result = fread(file_content, 1, file_size, file);

    if (((long) result) != file_size) {
        printf("Error while reading file content.");

        return 3;
    }

    Result output = parse(file_content);

    if (output.tag == Err) {
        printf("Parse error\n");

        return 4;
    }

    const Vector_Node nodes = output.ok._0;
    print(&nodes, 0);

    free(file_content);
    fclose(file);

    return 0;
}

/*
 * Local variables:
 * tab-width: 4
 * c-basic-offset: 4
 * End:
 */
