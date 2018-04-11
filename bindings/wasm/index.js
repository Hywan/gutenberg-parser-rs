const rust = import("./parser");

export class Block {
    constructor() {
        console.log("here");
    }
}

export function accumulate_block(block) {
    console.log(block)
}

rust.then(
    parser => {
        const data = ["<!-- wp:foo /-->", "hello"];

        for (let datum of data) {
            parser.root(datum);
        }
    }
);
