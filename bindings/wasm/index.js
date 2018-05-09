function parser() {
    const log = false;

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
        const buffer = new Uint8Array(module.memory.buffer.slice(start_pointer));
        const number_of_nodes = buffer[0];

        if (0 >= number_of_nodes) {
            return null;
        }

        log && console.log('number of nodes', number_of_nodes);

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
        log && console.group('read node');
        log && console.log('offset', offset);

        const node_type = buffer[offset];

        log && console.info('node type', node_type);

        // Block.
        if (1 === node_type) {
            const name_length = buffer[offset + 1];
            const attributes_length = buffer[offset + 2];
            const number_of_children = buffer[offset + 3];

            log && console.log('name length', name_length);
            log && console.log('attributes length', attributes_length);
            log && console.log('number of children', number_of_children);

            let payload_offset = offset + 4;
            let next_payload_offset = payload_offset + name_length;

            const name = text_decoder(buffer.slice(payload_offset, next_payload_offset));

            log && console.log('node name', name);

            payload_offset = next_payload_offset;
            next_payload_offset += attributes_length;

            const attributes = JSON.parse(text_decoder(buffer.slice(payload_offset, next_payload_offset)));

            log && console.log('attributes', attributes);

            payload_offset = next_payload_offset;
            let end_offset = payload_offset;

            const children = [];

            for (let i = 0; i < number_of_children; ++i) {
                const { last_offset, node } = readNode(buffer, payload_offset);

                payload_offset = end_offset = last_offset;
                children.push(node);
            }

            log && console.log('children', children);
            log && console.log('last offset', end_offset);

            log && console.groupEnd();

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

            log && console.log('phrase length', phrase_length);

            const phrase = text_decoder(buffer.slice(offset + 3, offset + 3 + phrase_length));

            log && console.log('phrase', phrase);

            log && console.groupEnd();

            return {
                last_offset: offset + 3 + phrase_length,
                node: new Phrase(phrase)
            }
        } else {
            log && console.error('unknown node type', node_type);
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

    const Module = {};
    const Parser = {
        root: function(datum) {
            performance.mark('input-preparing');

            const buffer = text_encoder(datum);
            const buffer_pointer = writeString(Module, buffer);

            performance.mark('parse-start');

            const output_pointer = Module.root(buffer_pointer, buffer.length);

            performance.mark('parse-stop');
            performance.mark('read-nodes-start');

            const result = readNodes(Module, output_pointer);

            performance.mark('read-nodes-stop');

            Module.dealloc(buffer_pointer, buffer.length);

            return result;
        }
    };

    return new function () {
        const wasm = fetchAndInstantiate('./gutenberg_post_parser.wasm', {});

        return (input) => {
            wasm.then(
                (module) => {
                    performance.mark('init');

                    if (undefined === Module.alloc) {
                        Module.alloc = module.exports.alloc;
                        Module.dealloc = module.exports.dealloc;
                        Module.root = module.exports.root;
                        Module.memory = module.exports.memory;
                    }

                    performance.mark('module-set');

                    const output = Parser.root(input);
                    log && console.table(output);
                    document.getElementById('output').value = JSON.stringify(output, null, 2);

                    performance.mark('shutdown');

                    performance.measure('global', 'init', 'shutdown');
                    performance.measure('preamble', 'input-preparing', 'parse-start');
                    performance.measure('parsing', 'parse-start', 'parse-stop');
                    performance.measure('decoding', 'read-nodes-start', 'read-nodes-stop');

                    console.table(
                        performance
                            .getEntriesByType('measure')
                            .map(
                                ({name, duration}) => {
                                    return {name, duration};
                                }
                            )
                    );

                    performance.clearMarks();
                    performance.clearMeasures();
                }
            );
        };
    };
};

document.addEventListener(
    'DOMContentLoaded',
    () => {
        const p = parser();
        p(document.getElementById('input').value)
    }
);
