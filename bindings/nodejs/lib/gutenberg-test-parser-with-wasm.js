#!/usr/bin/env node

const fs = require('fs');
const util = require('util');
const buffer = fs.readFileSync(__dirname + '/../../wasm/bin/gutenberg_post_parser.wasm');

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

function u8s_to_u32(p, o, q, r) {
    return (p << 24) | (o << 16) | (q << 8) | r;
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
    const number_of_nodes = u8s_to_u32(buffer[0], buffer[1], buffer[2], buffer[3]);

    if (0 >= number_of_nodes) {
        return null;
    }

    const nodes = [];
    let offset = 4;
    let end_offset;

    for (let i = 0; i < number_of_nodes; ++i) {
        const last_offset = readNode(buffer, offset, nodes);

        offset = end_offset = last_offset;
    }

    module.dealloc(start_pointer, start_pointer + end_offset);

    return nodes;
}

function readNode(buffer, offset, nodes) {
    const node_type = buffer[offset];

    // Block.
    if (1 === node_type) {
        const name_length = buffer[offset + 1];
        const attributes_length = u8s_to_u32(buffer[offset + 2], buffer[offset + 3], buffer[offset + 4], buffer[offset + 5]);
        const number_of_children = buffer[offset + 6];

        let payload_offset = offset + 7;
        let next_payload_offset = payload_offset + name_length;

        const name = text_decoder(buffer.slice(payload_offset, next_payload_offset));

        payload_offset = next_payload_offset;
        next_payload_offset += attributes_length;

        const attributes = JSON.parse(text_decoder(buffer.slice(payload_offset, next_payload_offset)));

        payload_offset = next_payload_offset;
        let end_offset = payload_offset;

        const children = [];

        for (let i = 0; i < number_of_children; ++i) {
            const last_offset = readNode(buffer, payload_offset, children);

            payload_offset = end_offset = last_offset;
        }

        nodes.push(new Block(name, attributes, children));

        return end_offset;
    }
    // Phrase.
    else if (2 === node_type) {
        const phrase_length = u8s_to_u32(buffer[offset + 1], buffer[offset + 2], buffer[offset + 3], buffer[offset + 4]);

        const phrase_offset = offset + 5;
        const phrase = text_decoder(buffer.slice(phrase_offset, phrase_offset + phrase_length));

        nodes.push(new Phrase(phrase));

        return phrase_offset + phrase_length;
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
