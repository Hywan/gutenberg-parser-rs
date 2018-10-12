function writeString(module, string_buffer) {
    const string_length = string_buffer.length;
    const pointer = module.alloc(string_length);

    const buffer = new Uint8ClampedArray(module.memory.buffer, pointer);

    for (let i = 0; i < string_length; i++) {
        buffer[i] = string_buffer[i]
    }

    return pointer;
}

function readNodes(module, start_pointer) {
    const buffer_length = new Uint32Array(module.memory.buffer, start_pointer, 1)[0];
    const payload_pointer = start_pointer + 4;

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

    module.dealloc_u32(start_pointer, buffer_length);

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
        const buffer_pointer = writeString(module, buffer);
        const output_pointer = module.root(buffer_pointer, buffer.length);
        const result = readNodes(module, output_pointer);

        module.input = null;

        console.log(buffer_pointer);
        console.log(buffer.length);

        console.log(new Uint8Array(module.memory.buffer, buffer_pointer, buffer.length));

        module.dealloc_u8(buffer_pointer, buffer.length);

        console.log(new Uint8Array(module.memory.buffer, buffer_pointer, buffer.length));

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
                    _module.dealloc_u8 = module.exports.dealloc_u8;
                    _module.dealloc_u32 = module.exports.dealloc_u32;
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
