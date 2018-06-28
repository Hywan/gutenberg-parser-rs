import { Gutenberg_Post_Parser } from './gutenberg_post_parser.mjs'

export class Gutenberg_Post_Parser_ASM extends Gutenberg_Post_Parser {
    /**
     * `module` is an object containing the following keys:
     *     * `root`, the function to run the parser,
     *     * `alloc`, the function to allocate memory,
     *     * `dealloc`, the function to free memory,
     *     * `memory`, the module memory.
     */
    constructor(Block, Phrase, module) {
        super(Block, Phrase);

        const self = this;

        this._Module = module;
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

    root(input) {
        return this._Parser.root(input);
    }
}
