#!/usr/bin/env node

const fs = require('fs');
const util = require('util');
const buffer = fs.readFileSync(__dirname + '/../../wasm/gutenberg_post_parser.wasm');

const text_encoder = new function () {
    const encoder = new util.TextEncoder('utf-8');

    return (string) => {
        return encoder.encode(string);
    };
}

const text_decoder = new function () {
    const decoder = new util.TextDecoder('utf-8');

    return (array_buffer) => {
        return decoder.decode(array_buffer);
    };
};

function writeString(module, string) {
    const string_buffer = text_encoder(string);
    const string_length = string_buffer.length;
    const pointer = module.alloc(string_length + 1);

    const buffer = new Uint8Array(module.memory.buffer);

    for (let i = 0; i < string_length; i++) {
        buffer[pointer + i] = string_buffer[i]
    }

    buffer[pointer + string_length] = 0;

    return pointer;
}

function readNodes(module, start_pointer) {
    const buffer = module.memory.buffer;
    const [number_of_nodes] = new Uint8Array(buffer.slice(start_pointer, start_pointer + 1));

    if (0 >= number_of_nodes) {
        return null;
    }

    const nodes = [];
    let pointer = start_pointer + 1;
    let end_pointer;

    for (let i = 0; i < number_of_nodes; ++i) {
        const { last_pointer, node } = readNode(buffer, pointer);

        pointer = end_pointer = last_pointer;
        nodes.push(node);
    }

    module.dealloc(start_pointer, end_pointer);

    return nodes;
}

function readNode(buffer, pointer) {
    const [node_type] = new Uint8Array(buffer.slice(pointer, pointer + 1));

    // Block.
    if (1 === node_type) {
        const [name_length, attributes_length, number_of_children] = new Uint8Array(buffer.slice(pointer + 1, pointer + 4));
        const payload = buffer.slice(pointer + 4);

        let offset = 0;
        let next_offset = name_length;

        const name = text_decoder(payload.slice(offset, next_offset));

        offset = next_offset;
        next_offset = next_offset + attributes_length;

        const attributes = JSON.parse(text_decoder(payload.slice(offset, next_offset)));

        offset = pointer + 4 + next_offset;
        let end_pointer = offset;

        const children = [];

        for (let i = 0; i < number_of_children; ++i) {
            const { last_pointer, node } = readNode(buffer, offset);

            offset = end_pointer = last_pointer;
            children.push(node);
        }

        return {
            last_pointer: end_pointer,
            node: new Block(name, attributes, children)
        };
    }
    // Phrase.
    else if (2 === node_type) {
        const [phrase_length] = new Uint8Array(buffer.slice(pointer + 1, pointer + 2));

        const phrase = text_decoder(buffer.slice(pointer + 2, pointer + 2 + phrase_length));

        return {
            last_pointer: pointer + 2 + phrase_length,
            node: new Phrase(phrase)
        }
    }
}

class Block {
    constructor(name, attributes, children) {
        this.blockName = name;
        this.attrs = attributes;
        this.innerBlocks = [];
        this.innerHTML = '';

        for (let child of children) {
            if (child instanceof Block) {
                this.innerBlocks.push(child);
            } else if (child instanceof Phrase) {
                this.innerHTML += child.innerHTML;
            }
        }
    }
}

class Phrase {
    constructor(phrase) {
        this.attrs = {};
        this.innerHTML = phrase;
    }
}

let Module = {};
let Parser = {
    root: function(datum) {
        let string_pointer = writeString(Module, datum);
        let output_pointer = Module.root(string_pointer, datum.length);
        let result = readNodes(Module, output_pointer);
        Module.dealloc(string_pointer, datum.length + 1);

        return result;
    }
};

WebAssembly
    .instantiate(buffer, {})
    .then(
        (object) => {
            const module = object.instance;
            Module.alloc = module.exports.alloc;
            Module.dealloc = module.exports.dealloc;
            Module.root = module.exports.root;
            Module.memory = module.exports.memory;

            const stdin = process.stdin;
            const stdout = process.stdout;
            let input = '';

            stdin.setEncoding('utf-8');
            stdin.on(
                'readable',
                () => {
                    let chunk;

                    while (chunk = stdin.read()) {
                        input += chunk;
                    }
                }
            );
            stdin.on(
                'end',
                () => {
                    stdout.write(
                        JSON.stringify(
                            Parser.root(input)
                        )
                    );
                }
            );
        }
    );
