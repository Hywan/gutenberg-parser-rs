const rust = import("./parser");

export function accumulate_block(block) {
    console.log(block)
}

export class Block {
    constructor(block_as_json) {
        const block = JSON.parse(block_as_json);
        this.name = block.name;
        this.attributes = block.attributes;
        this.inner_blocks = block.inner_blocks;
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
