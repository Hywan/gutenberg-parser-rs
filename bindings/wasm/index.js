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

function writeString(module, string) {
    const utf8Encoder = new TextEncoder("utf-8");
    const string_buffer = utf8Encoder.encode(string);
    const string_length = string_buffer.length;
    const pointer = module.alloc(string_length + 1);

    const buffer = new Uint8Array(module.memory.buffer);

    for (let i = 0; i < string_length; i++) {
        buffer[pointer + i] = string_buffer[i]
    }

    buffer[pointer + string_length] = 0;

    return pointer;
}

function readBlock(module, pointer) {
    const buffer = module.memory.buffer;
    const [name_length, attributes_length, inner_blocks_length] = new Uint8Array(buffer.slice(pointer, pointer + 3));
    const payload = buffer.slice(pointer + 3);

    let decoder = new function () {
        const decoder = new TextDecoder("utf-8");

        return (array_buffer) => {
            return decoder.decode(array_buffer);
        };
    };

    let offset = 0;
    let next_offset = name_length;

    const name = decoder(payload.slice(offset, next_offset));

    offset = next_offset;
    next_offset = next_offset + attributes_length;

    const attributes = JSON.parse(decoder(payload.slice(offset, next_offset)));

    offset = next_offset;
    next_offset = next_offset + inner_blocks_length;

    const inner_blocks = decoder(payload.slice(offset, next_offset));

    module.dealloc_str(pointer + next_offset + 3);

    return new Block(name, attributes, inner_blocks);
}

class Block {
    constructor(name, attributes, inner_blocks) {
        this.name = name;
        this.attributes = attributes;
        this.inner_blocks = inner_blocks;
    }
}

let Module = {};
let Parser = {
    root: function(datum) {
        let string_pointer = writeString(Module, datum);
        let output_pointer = Module.root(string_pointer);
        let result = readBlock(Module, output_pointer);
        Module.dealloc_str(string_pointer);

        return result;
    }
};

fetchAndInstantiate("./parser_bg.wasm", {})
    .then(
        (module) => {
            console.log(module);
            Module.alloc = module.exports.alloc;
            Module.dealloc = module.exports.dealloc;
            Module.dealloc_str = module.exports.dealloc_str;
            Module.root = module.exports.root;
            Module.memory = module.exports.memory;

            console.log(Parser.root(`<!-- wp:foo {"bar": "qux"} /-->`));
        }
    );
