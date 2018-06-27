import { Gutenberg_Post_Parser } from "./gutenberg_post_parser.mjs";

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
        const parser = new Gutenberg_Post_Parser(Block, Phrase, './gutenberg_post_parser.wasm');

        parser.root(document.getElementById('input').value).then(
            (output) => {
                document.getElementById('output').value = JSON.stringify(output, null, 2);
            }
        );
    }
);
