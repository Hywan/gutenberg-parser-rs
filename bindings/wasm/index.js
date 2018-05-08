function fetchAndInstantiate(url, importObject) {
    return fetch(url)
        .then(
            (response) => response.arrayBuffer()
        )
        .then(
            (bytes) => WebAssembly.instantiate(bytes, importObject)
        )
        .then(
            (results) => results.instance
        );
}

const text_encoder = new function () {
    const encoder = new TextEncoder();

    return (string) => {
        return encoder.encode(string);
    };
}

const text_decoder = new function () {
    const decoder = new TextDecoder('utf-8');

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
    const buffer = module.memory.buffer;
    const [number_of_nodes] = new Uint8Array(buffer.slice(start_pointer, start_pointer + 1));

    if (0 >= number_of_nodes) {
        return null;
    }

    console.log('number of nodes', number_of_nodes);

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
    console.group('read node');

    console.log('pointer', pointer);

    const [node_type] = new Uint8Array(buffer.slice(pointer, pointer + 1));

    console.info('node type', node_type);

    // Block.
    if (1 === node_type) {
        const [name_length, attributes_length, number_of_children] = new Uint8Array(buffer.slice(pointer + 1, pointer + 4));
        const payload = buffer.slice(pointer + 4);

        console.log('name length', name_length);
        console.log('attributes length', attributes_length);
        console.log('number of children', number_of_children);

        let offset = 0;
        let next_offset = name_length;

        const name = text_decoder(payload.slice(offset, next_offset));

        console.log('node name', name);

        offset = next_offset;
        next_offset = next_offset + attributes_length;

        const attributes = JSON.parse(text_decoder(payload.slice(offset, next_offset)));

        console.log('attributes', attributes);

        offset = pointer + 4 + next_offset;
        let end_pointer = offset;

        const children = [];

        for (let i = 0; i < number_of_children; ++i) {
            const { last_pointer, node } = readNode(buffer, offset);

            offset = end_pointer = last_pointer;
            children.push(node);
        }

        console.log('children', children);
        console.log('last pointer', end_pointer);

        console.groupEnd();

        return {
            last_pointer: end_pointer,
            node: new Block(name, attributes, children)
        };
    }
    // Phrase.
    else if (2 === node_type) {
        const [phrase_length_0, phrase_length_1] = new Uint8Array(buffer.slice(pointer + 1, pointer + 3));
        const phrase_length = u8s_to_u16(phrase_length_0, phrase_length_1);

        console.log('phrase length', phrase_length);

        const phrase = text_decoder(buffer.slice(pointer + 3, pointer + 3 + phrase_length));

        console.log('phrase', phrase);

        console.groupEnd();

        return {
            last_pointer: pointer + 3 + phrase_length,
            node: new Phrase(phrase)
        }
    } else {
        console.error('unknown node type', node_type);
    }
}

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

fetchAndInstantiate("./gutenberg_post_parser.wasm", {})
    .then(
        (module) => {
            Module.alloc = module.exports.alloc;
            Module.dealloc = module.exports.dealloc;
            Module.root = module.exports.root;
            Module.memory = module.exports.memory;

            const input = document.getElementById('input').value;
            const output = Parser.root(input);
            console.log(output);
            document.getElementById('output').value = JSON.stringify(output, null, 2);
        }
    );
