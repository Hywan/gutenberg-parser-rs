#!/usr/bin/env node --experimental-modules
import {Gutenberg_Post_Parser} from './gutenberg_post_parser.mjs'
import fs from 'fs';
import util from 'util';
import process from 'process';

class Gutenberg_Post_Parser_NodeJS extends Gutenberg_Post_Parser {
    constructor(Block, Phrase, wasmURL) {
        super(Block, Phrase, wasmURL, new util.TextEncoder(), new util.TextDecoder());
    }

    instantiateWASM(url, importObject) {
        const buffer = fs.readFileSync(url);

        return this._wasm = WebAssembly.instantiate(buffer, {}).then(obj => obj.instance);
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

const stdin = process.stdin;
const stdout = process.stdout;
const stderr = process.stderr;

function usage() {
    return 'USAGE:' + "\n" +
        '    ' + process.argv0 + ' [FLAGS] [INPUT]' + "\n\n" +
        'FLAGS:' + "\n" +
        '    -d, --emit-debug    Compile the AST into JS debug format (default).' + "\n" +
        '    -j, --emit-json     Compile the AST into JSON format.' + "\n" +
        '    -h, --help          Prints help information.' + "\n\n" +
        'ARGS:' + "\n" +
        '    <INPUT>    File containing the input.' + "\n";
}

if (2 >= process.argv.length) {
    stderr.write(usage());

    process.exit(1);
}

const parser = new Gutenberg_Post_Parser_NodeJS(Block, Phrase, './gutenberg_post_parser.wasm');

let input = '';
let emit = 'debug';

process.argv.slice(2).forEach(
    (argument_value) => {
        switch (argument_value) {
            case '-h':
            case '--help':
                stderr.write(usage());

                process.exit(2);

            case '-d':
            case '--emit-debug':
                emit = 'debug';

                break;

            case '-j':
            case '--emit-json':
                emit = 'json';

                break;

            default:
                if (argument_value && '-' === argument_value[0]) {
                    stderr.write('Argument `' + argument_value + '` is invalid.' + "\n\n");
                    stderr.write(usage());

                    process.exit(3);
                }

                input = argument_value;
        }
    }
);

if (!input) {
    stderr.write('File is missing.' + "\n\n");
    stderr.write(usage());

    process.exit(4);
}

try {
    fs.accessSync(input, fs.constants.R_OK)
} catch (e) {
    stderr.write('File `' + input + '` does not exist, or is not readable.' + "\n");

    process.exit(5);
}

const content = fs.readFileSync(input, { encoding: 'utf-8' });

parser.root(content).then(
    (output) => {
        switch (emit) {
            case 'debug':
                console.log(output);

                break;

            case 'json':
                stdout.write(JSON.stringify(output, null, 4));

                break;
        }
    }
);
