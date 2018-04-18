export function accumulate_block(block) {
    console.log(block)
}

export class Block {
    constructor(block_as_vec_u8) {
        const buffer = block_as_vec_u8.buffer;
        const [name_length, attributes_length, inner_blocks_length] = new Uint8Array(buffer.slice(0, 3));
        const payload = buffer.slice(3);

        let decoder = new function () {
            const decoder = new TextDecoder('utf-8');

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
        next_offset = next_offset + attributes_length;

        const inner_blocks = decoder(payload.slice(offset, next_offset));

        this.name = name;
        this.attributes = attributes;
        this.inner_blocks = inner_blocks;
    }
}
