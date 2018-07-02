import { Gutenberg_Post_Parser_ASM } from './gutenberg_post_parser.asm.mjs';

class Block {
    constructor(name, attributes, children) {
        this.name = name;
        this.attributes = attributes;
        this.children = children;
    }
}

class Phrase {
    constructor(phrase) {
        this.phrase = phrase;
    }
}

document.addEventListener(
    'DOMContentLoaded',
    () => {
        const parser = new Gutenberg_Post_Parser_ASM(Block, Phrase, GUTENBERG_POST_PARSER_ASM_MODULE());

        document.getElementById('output').value = JSON.stringify(
            parser.root(document.getElementById('input').value),
            null,
            2
        );
    }
);
