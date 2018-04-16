const rust = import("./parser");

export function accumulate_block(block) {
    console.log(block)
}

export class Block {
    constructor(block_as_json) {
        console.log(JSON.parse(block_as_json));
    }
}

rust.then(
    parser => {
        const data = [`<!-- wp:foo {"bar": "qux"} /-->`, `hello`];

        for (let datum of data) {
            parser.root(datum);
        }
    }
);
