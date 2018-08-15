export class Gutenberg_Post_Parser {
    constructor(Block, Phrase, wasmURL, textEncoder, textDecoder) {
        this.Block = Block;
        this.Phrase = Phrase;

        if (undefined !== wasmURL) {
            this.instantiateWASM(wasmURL, {});
        }

        this._encoder = textEncoder || new TextEncoder();
        this._decoder = textDecoder || new TextDecoder('utf-8');

        const self = this;

        this._Module = {};
        this._Parser = {
            root: function(datum) {
                const buffer = self.text_encoder(datum);
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

    text_decoder(array_buffer) {
        return this._decoder.decode(array_buffer);
    }

    u8s_to_u32(o, p, q, r) {
        return (o << 24) | (p << 16) | (q << 8) | r;
    }

    _writeString(module, string_buffer) {
        const string_length = string_buffer.length;
        const pointer = module.alloc(string_length);

        const buffer = new Uint8Array(module.memory.buffer);

        for (let i = 0; i < string_length; i++) {
            buffer[pointer + i] = string_buffer[i]
        }

        return pointer;
    }

    _readNodes(module, start_pointer) {
        const buffer_length = this.u8s_to_u32(...new Uint8Array(module.memory.buffer.slice(start_pointer, start_pointer + 4)));

        const payload_pointer = start_pointer + 4;

        const buffer = new Uint8Array(module.memory.buffer.slice(payload_pointer, payload_pointer + buffer_length));
        const number_of_nodes = this.u8s_to_u32(buffer[0], buffer[1], buffer[2], buffer[3]);

        if (0 >= number_of_nodes) {
            return null;
        }

        const nodes = [];
        let offset = 4;
        let end_offset;

        for (let i = 0; i < number_of_nodes; ++i) {
            const last_offset = this._readNode(buffer, offset, nodes);

            offset = end_offset = last_offset;
        }

        module.dealloc(start_pointer, start_pointer + end_offset);

        return nodes;
    }

    _readNode(buffer, offset, nodes) {
        const node_type = buffer[offset];

        // Block.
        if (1 === node_type) {
            const name_length = buffer[offset + 1];
            const attributes_length = this.u8s_to_u32(buffer[offset + 2], buffer[offset + 3], buffer[offset + 4], buffer[offset + 5]);
            const number_of_children = buffer[offset + 6];

            let payload_offset = offset + 7;
            let next_payload_offset = payload_offset + name_length;

            const name = this.text_decoder(buffer.slice(payload_offset, next_payload_offset));

            payload_offset = next_payload_offset;
            next_payload_offset += attributes_length;

            const attributes = JSON.parse(this.text_decoder(buffer.slice(payload_offset, next_payload_offset)));

            payload_offset = next_payload_offset;
            let end_offset = payload_offset;

            const children = [];

            for (let i = 0; i < number_of_children; ++i) {
                const last_offset = this._readNode(buffer, payload_offset, children);

                payload_offset = end_offset = last_offset;
            }

            nodes.push(new this.Block(name, attributes, children));

            return end_offset;
        }
        // Phrase.
        else if (2 === node_type) {
            const phrase_length = this.u8s_to_u32(buffer[offset + 1], buffer[offset + 2], buffer[offset + 3], buffer[offset + 4]);
            const phrase_offset = offset + 5;
            const phrase = this.text_decoder(buffer.slice(phrase_offset, phrase_offset + phrase_length));

            nodes.push(new this.Phrase(phrase));

            return phrase_offset + phrase_length;
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
