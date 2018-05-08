#!/usr/bin/env node

const fs = require('fs');
const util = require('util');
const buffer = fs.readFileSync(__dirname + '/../../wasm/gutenberg_post_parser.wasm');

const text_encoder = new function () {
    const encoder = new util.TextEncoder();

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

function u8s_to_u16(p, o) {
    return (p << 8) | o;
}

function writeString(module, string_buffer) {
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
    const buffer = new Uint8Array(module.memory.buffer.slice(start_pointer));
    const number_of_nodes = buffer[0];

    if (0 >= number_of_nodes) {
        return null;
    }

    const nodes = [];
    let offset = 1;
    let end_offset;

    for (let i = 0; i < number_of_nodes; ++i) {
        const { last_offset, node } = readNode(buffer, offset);

        offset = end_offset = last_offset;
        nodes.push(node);
    }

    module.dealloc(start_pointer, start_pointer + end_offset);

    return nodes;
}

function readNode(buffer, offset) {
    const node_type = buffer[offset];

    // Block.
    if (1 === node_type) {
        const name_length = buffer[offset + 1];
        const attributes_length = buffer[offset + 2];
        const number_of_children = buffer[offset + 3];

        let payload_offset = offset + 4;
        let next_payload_offset = payload_offset + name_length;

        const name = text_decoder(buffer.slice(payload_offset, next_payload_offset));

        payload_offset = next_payload_offset;
        next_payload_offset += attributes_length;

        const attributes = JSON.parse(text_decoder(buffer.slice(payload_offset, next_payload_offset)));

        payload_offset = next_payload_offset;
        let end_offset = payload_offset;

        const children = [];

        for (let i = 0; i < number_of_children; ++i) {
            const { last_offset, node } = readNode(buffer, payload_offset);

            payload_offset = end_offset = last_offset;
            children.push(node);
        }

        return {
            last_offset: end_offset,
            node: new Block(name, attributes, children)
        };
    }
    // Phrase.
    else if (2 === node_type) {
        const phrase_length_0 = buffer[offset + 1];
        const phrase_length_1 = buffer[offset + 2];
        const phrase_length = u8s_to_u16(phrase_length_0, phrase_length_1);

        const phrase = text_decoder(buffer.slice(offset + 3, offset + 3 + phrase_length));

        return {
            last_offset: offset + 3 + phrase_length,
            node: new Phrase(phrase)
        }
    } else {
        console.error('unknown node type', node_type);
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
        const buffer = text_encoder(datum);
        const buffer_pointer = writeString(Module, buffer);
        const output_pointer = Module.root(buffer_pointer, buffer.length);
        const result = readNodes(Module, output_pointer);
        Module.dealloc(buffer_pointer, buffer.length);

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
