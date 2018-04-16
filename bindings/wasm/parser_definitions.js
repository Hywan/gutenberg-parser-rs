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
