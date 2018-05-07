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

    const [name_length, attributes_length, number_of_children] = new Uint8Array(buffer.slice(pointer, pointer + 3));
    const payload = buffer.slice(pointer + 3);

    console.log('name length', name_length);
    console.log('attributes length', attributes_length);
    console.log('number of children', number_of_children);

    let decoder = new function () {
        const decoder = new TextDecoder("utf-8");

        return (array_buffer) => {
            return decoder.decode(array_buffer);
        };
    };

    let offset = 0;
    let next_offset = name_length;

    const name = decoder(payload.slice(offset, next_offset));

    console.log('node name', name);

    offset = next_offset;
    next_offset = next_offset + attributes_length;

    const attributes = JSON.parse(decoder(payload.slice(offset, next_offset)));

    console.log('attributes', attributes);

    offset = pointer + 3 + next_offset;
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
        node: new Node(name, attributes, children)
    };
}

class Node {
    constructor(name, attributes, children) {
        this.name = name;
        this.attributes = attributes;
        this.children = children;
    }
}

let Module = {};
let Parser = {
    root: function(datum) {
        let string_pointer = writeString(Module, datum);
        let output_pointer = Module.root(string_pointer, datum.length);
        let result = readNodes(Module, output_pointer);
        Module.dealloc(string_pointer, datum.length + 1);

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

            console.log(Parser.root(`<!-- wp:a1 {"bar": "qux"} --><!-- wp:a1-1 {"x": "y"} /--><!-- wp:a1-2 /--><!-- /wp:foo --><!-- wp:a2 /-->`));
        }
    );
