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
            const char *namespace = block.namespace;
            const char *name = block.name;

            printf("%*.*sblock\n", depth, depth, " ");

            printf("%*.*s    %s/%s\n", depth, depth, " ", namespace, name);

            free((void*) namespace);
            free((void*) name);

            if (block.attributes.tag == Some) {
                const char *attributes = block.attributes.some._0;

                printf("%*.*s    %s\n", depth, depth, " ", attributes);

                free((void*) attributes);
            }

            const Vector_Node* children = (const Vector_Node*) (block.children);

            print(children, depth + 4);

            free((void*) children);
        } else if (node.tag == Phrase) {
            const char *phrase = node.phrase._0;

            printf("%*.*sphrase\n", depth, depth, " ");
            printf("%*.*s    %s\n", depth, depth, " ", phrase);

            free((void*) phrase);
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
        printf("Cannot read given file.");

        return 2;
    }

    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    rewind(file);

    char* file_content = (char*) malloc(file_size * sizeof(char*));
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
