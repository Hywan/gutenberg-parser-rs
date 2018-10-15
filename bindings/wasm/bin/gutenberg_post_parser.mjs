function writeString(module, string_buffer, string_buffer_capacity) {
    const pointer = module.alloc(string_buffer_capacity);

    const buffer = new Uint8ClampedArray(module.memory.buffer, pointer);

    for (let i = 0; i < string_buffer_capacity; i++) {
        buffer[i] = string_buffer[i]
    }

    return pointer;
}

function readNodes(module, start_pointer) {
    console.log('read nodes', start_pointer);

    const buffer_properties = new Uint32Array(module.memory.buffer, start_pointer, 2);
    const buffer_capacity = buffer_properties[0];
    const buffer_length = buffer_properties[1] / 4;
    const payload_pointer = start_pointer + 8;

    const buffer = new Uint32Array(module.memory.buffer, payload_pointer, buffer_length);
    const number_of_nodes = buffer[0];

    if (0 >= number_of_nodes) {
        return null;
    }

    const nodes = [];
    let offset = 1;

    for (let i = 0; i < number_of_nodes; ++i) {
        offset = readNode(module, buffer, offset, nodes);
    }

    module.dealloc(start_pointer, buffer_capacity);

    return nodes;
}

function readNode(module, buffer, offset, nodes) {
    const node_type = buffer[offset];
    offset += 1;

    switch (node_type) {
        case 1: // Block.
            const name_length = buffer[offset];
            offset += 1;

            const name =
                buffer
                    .subarray(offset, offset + name_length)
                    .reduce(
                        (accumulator, value) => accumulator + String.fromCharCode(value),
                        ''
                    );

            offset += name_length;

            const attributes_offset = buffer[offset    ];
            const attributes_length = buffer[offset + 1];

            const attributes =
                0 === attributes_length
                    ? null
                    : JSON.parse(module.input.substr(attributes_offset, attributes_length));

            const number_of_children = buffer[offset + 2];
            const children = [];

            offset += 3;

            for (let i = 0; i < number_of_children; ++i) {
                offset = readNode(module, buffer, offset, children);
            }

            nodes.push(new module.Block(name, attributes, children));

            return offset;

        case 2: // Phrase.
            const phrase_offset = buffer[offset    ];
            const phrase_length = buffer[offset + 1];

            const phrase = module.input.substr(phrase_offset, phrase_length);

            nodes.push(new module.Phrase(phrase));

            return offset + 2;

        default:
            console.error('unknown node type', node_type);
    }
}

export class Gutenberg_Post_Parser {
    constructor(Block, Phrase, wasmURL, textEncoder) {
        this.Block = Block;
        this.Phrase = Phrase;

        if (undefined !== wasmURL) {
            this.instantiateWASM(wasmURL, {});
        }

        this._encoder = textEncoder || new TextEncoder();
    }

    _parse(module) {
        const buffer = this._encoder.encode(module.input);
        const buffer_length = buffer.length;
        const buffer_pointer = writeString(module, buffer, buffer_length);

        const output_pointer = module.root(buffer_pointer, buffer_length);
        const result = readNodes(module, output_pointer);

        module.input = null;

        module.dealloc(buffer_pointer, buffer_length);

        return result;
    }

    instantiateWASM(url, importObject) {
        return this._wasm = WebAssembly.instantiateStreaming(fetch(url), importObject).then(obj => obj.instance);
    }

    root(input) {
        const _module = {};

        return this._wasm.then(
            (module) => {
                if (undefined === _module.alloc) {
                    _module.alloc = module.exports.alloc;
                    _module.dealloc = module.exports.dealloc;
                    _module.root = module.exports.root;
                    _module.memory = module.exports.memory;
                    _module.Block = this.Block;
                    _module.Phrase = this.Phrase;
                }

                _module.input = input;

                return this._parse(_module);
            }
        );
    }
}
