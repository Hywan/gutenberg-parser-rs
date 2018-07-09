<?php

const EXPRESSION = <<<'PCRE'
`
    \G
    (?<block_list>
        (?|
            # Block

            ## Block opening

            <!--
            [ \n\r\t]*
            wp:(?<block_name>(?:[a-z][a-z0-9\-]*/)?[a-z][a-z0-9\-]*)
            [ \n\r\t]+
            (?<block_attributes>{[^}]+})?
            [ \n\r\t]*
            (?:
                ## Void

                /-->

                (*MARK:void_block)

                |

                ## Balanced

                -->

                ## Block children

                (?<block_children>
                    (?&block_list)
                )*

                ## Block closing

                <!--[ \n\r\t]*/wp:\g{block_name}[ \n\r\t]*-->

                (*MARK:balanced_block)
            )

            |

            # Phrase

            .+?(?=<!--[ \n\r\t]*/?wp:)

            (*MARK:phrase)

            |

            # Phrase tail

            .+

            (*MARK:phrase_tail)
        )
    )
`xsm
PCRE;

/*
*/
const DATA = <<<'DATA'
first
<!-- wp:foo --><!-- wp:bar -->blabla<!-- /wp:bar --><!-- /wp:foo -->
<!-- wp:baz {"a": "b"} -->yolo<!-- /wp:baz -->
<!-- wp:q/ux {"x": "y"} /-->
haa
<!-- more -->
tail tail
DATA;

class Block
{
    public $name;
    public $attributes;
    public $children;
}

class Phrase
{
    public $content;
}

function parse(string $data)
{
    $offset = 0;
    $max_offset = strlen($data);

    do {
        $preg = preg_match(
            EXPRESSION,
            $data,
            $matches,
            0,
            $offset
        );

        if (0 === $preg || empty($matches)) {
            break;
        }

        switch ($matches['MARK']) {
            case 'phrase':
            case 'phrase_tail':
                $phrase = new Phrase();
                $phrase->content = $matches[1];

                yield $phrase;

                break;

            case 'void_block':
                $block = new Block();
                $block->name = $matches['block_name'];

                if (!empty($matches['block_attributes'])) {
                    $block->attributes = json_decode($matches['block_attributes']);
                }

                yield $block;

                break;

            case 'balanced_block':
                $block = new Block();
                $block->name = $matches['block_name'];

                if (!empty($matches['block_attributes'])) {
                    $block->attributes = json_decode($matches['block_attributes']);
                }

                if (!empty($matches['block_children'])) {
                    $block->children = $matches['block_children'];
                }

                yield $block;

                break;
        }

        $offset += strlen($matches[0]);
    } while ($offset < $max_offset);
}

function unfold(string $data, int $depth = 0)
{
    $output = [];

    foreach (parse($data) as $node) {
        if ($node instanceof Block && !empty($node->children)) {
            $node->children = unfold($node->children);
        }

        $output[] = $node;
    }

    return $output;
}

echo '# ', str_replace("\n", "\n" . '# ', DATA), "\n\n";

echo json_encode(unfold(DATA), JSON_PRETTY_PRINT);
