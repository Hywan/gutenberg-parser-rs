export class Gutenberg_Post_Parser {
    constructor(Block, Phrase, wasmURL, textEncoder) {
        this.Block = Block;
        this.Phrase = Phrase;

        if (undefined !== wasmURL) {
            this.instantiateWASM(wasmURL, {});
        }

        this._encoder = textEncoder || new TextEncoder();

        const self = this;

        this._Module = {};
        this._Parser = {
            root: datum => {
                const buffer = self.text_encoder(datum);

                self._input = datum;

                const buffer_pointer = self._writeString(self._Module, buffer);
                const output_pointer = self._Module.root(buffer_pointer, buffer.length);
                const result = self._readNodes(self._Module, output_pointer);

                self._Module.dealloc(buffer_pointer, buffer.length);

                return result;
            }
        };
    }

    instantiateWASM(url, importObject) {
        return this._wasm = WebAssembly.instantiateStreaming(fetch(url), importObject).then(obj => obj.instance);
    }

    text_encoder(string) {
        return this._encoder.encode(string);
    }

    u8s_to_u32(o, p, q, r) {
        return (o << 24) | (p << 16) | (q << 8) | r;
    }

    _writeString(module, string_buffer) {
        const string_length = string_buffer.length;
        const pointer = module.alloc(string_length);

        const buffer = new Uint8ClampedArray(module.memory.buffer);

        for (let i = 0; i < string_length; i++) {
            buffer[pointer + i] = string_buffer[i]
        }

        return pointer;
    }

    _readNodes(module, start_pointer) {
        const buffer_length = this.u8s_to_u32(...new Uint8ClampedArray(module.memory.buffer.slice(start_pointer, start_pointer + 4)));

        const payload_pointer = start_pointer + 4;

        const buffer = new Uint8ClampedArray(module.memory.buffer.slice(payload_pointer, payload_pointer + buffer_length));
        const number_of_nodes = this.u8s_to_u32(buffer[0], buffer[1], buffer[2], buffer[3]);

        if (0 >= number_of_nodes) {
            return null;
        }

        const nodes = [];
        let offset = 4;

        for (let i = 0; i < number_of_nodes; ++i) {
            offset = this._readNode(buffer, offset, nodes);
        }

        module.dealloc(start_pointer, start_pointer + offset);

        return nodes;
    }

    _readNode(buffer, offset, nodes) {
        const node_type = buffer[offset];

        // Block.
        if (1 === node_type) {
            const name_length = buffer[offset + 1];
            offset += 2;

            const name =
                buffer
                    .subarray(offset, offset + name_length)
                    .reduce(
                        (accumulator, value) => {
                            return accumulator + String.fromCharCode(value);
                        },
                        ''
                    );

            offset += name_length;
            const attributes_offset = this.u8s_to_u32(buffer[offset    ], buffer[offset + 1], buffer[offset + 2], buffer[offset + 3]);
            const attributes_length = this.u8s_to_u32(buffer[offset + 4], buffer[offset + 5], buffer[offset + 6], buffer[offset + 7]);

            const attributes =
                0 === attributes_length
                    ? null
                    : JSON.parse(this._input.substr(attributes_offset, attributes_length));

            offset += 8;
            const number_of_children = buffer[offset];
            offset += 1;

            const children = [];

            for (let i = 0; i < number_of_children; ++i) {
                offset = this._readNode(buffer, offset, children);
            }

            nodes.push(new this.Block(name, attributes, children));

            return offset;
        }
        // Phrase.
        else if (2 === node_type) {
            const phrase_offset = this.u8s_to_u32(buffer[offset + 1], buffer[offset + 2], buffer[offset + 3], buffer[offset + 4]);
            const phrase_length = this.u8s_to_u32(buffer[offset + 5], buffer[offset + 6], buffer[offset + 7], buffer[offset + 8]);

            const phrase = this._input.substr(phrase_offset, phrase_length);

            nodes.push(new this.Phrase(phrase));

            return offset + 9;
        } else {
            console.error('unknown node type', node_type);
        }
    }

    root(input) {
        return this._wasm.then(
            (module) => {
                if (undefined === this._Module.alloc) {
                    this._Module.alloc = module.exports.alloc;
                    this._Module.dealloc = module.exports.dealloc;
                    this._Module.root = module.exports.root;
                    this._Module.memory = module.exports.memory;
                }

                return this._Parser.root(input);
            }
        );
    }
}
