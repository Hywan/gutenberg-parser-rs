function parser() {
    const log = false;
    const bench = false;

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

        log && console.log('number of nodes', number_of_nodes);

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
        log && console.group('read node');
        log && console.log('offset', offset);

        const node_type = buffer[offset];

        log && console.info('node type', node_type);

        // Block.
        if (1 === node_type) {
            const name_length = buffer[offset + 1];
            const attributes_length = u8s_to_u32(buffer[offset + 2], buffer[offset + 3], buffer[offset + 4], buffer[offset + 5]);
            const number_of_children = buffer[offset + 6];

            log && console.log('name length', name_length);
            log && console.log('attributes length', attributes_length);
            log && console.log('number of children', number_of_children);

            let payload_offset = offset + 7;
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
                const last_offset = readNode(buffer, payload_offset, children);

                payload_offset = end_offset = last_offset;
            }

            log && console.log('children', children);
            log && console.log('last offset', end_offset);

            log && console.groupEnd();

            nodes.push(new Block(name, attributes, children));

            return end_offset;
        }
        // Phrase.
        else if (2 === node_type) {
            const phrase_length = u8s_to_u32(buffer[offset + 1], buffer[offset + 2], buffer[offset + 3], buffer[offset + 4]);

            log && console.log('phrase length', phrase_length);

            const phrase_offset = offset + 5;
            const phrase = text_decoder(buffer.slice(phrase_offset, phrase_offset + phrase_length));

            log && console.log('phrase', phrase);

            log && console.groupEnd();

            nodes.push(new Phrase(phrase));

            return phrase_offset + phrase_length;
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
            bench && performance.mark('input-preparing');

            const buffer = text_encoder(datum);
            const buffer_pointer = writeString(Module, buffer);

            bench && performance.mark('parse-start');

            const output_pointer = Module.root(buffer_pointer, buffer.length);

            bench && performance.mark('parse-stop');
            bench && performance.mark('read-nodes-start');

            const result = readNodes(Module, output_pointer);

            bench && performance.mark('read-nodes-stop');

            Module.dealloc(buffer_pointer, buffer.length);

            return result;
        }
    };

    return new function () {
        const wasm = fetchAndInstantiate('./gutenberg_post_parser.wasm', {});

        return (input) => {
            wasm.then(
                (module) => {
                    bench && performance.mark('init');

                    if (undefined === Module.alloc) {
                        Module.alloc = module.exports.alloc;
                        Module.dealloc = module.exports.dealloc;
                        Module.root = module.exports.root;
                        Module.memory = module.exports.memory;
                    }

                    bench && performance.mark('module-set');

                    const output = Parser.root(input);
                    log && console.table(output);
                    document.getElementById('output').value = JSON.stringify(output, null, 2);

                    bench && performance.mark('shutdown');

                    bench && performance.measure('global', 'init', 'shutdown');
                    bench && performance.measure('preamble', 'input-preparing', 'parse-start');
                    bench && performance.measure('parsing', 'parse-start', 'parse-stop');
                    bench && performance.measure('decoding', 'read-nodes-start', 'read-nodes-stop');

                    bench && console.table(
                        performance
                            .getEntriesByType('measure')
                            .map(
                                ({name, duration}) => {
                                    return {name, duration};
                                }
                            )
                    );

                    bench && performance.clearMarks();
                    bench && performance.clearMeasures();
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
