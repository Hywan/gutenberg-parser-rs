#!/usr/bin/env node
const gutenberg_post_parser = require('../native');
const fs = require('fs');
const process = require('process');

const stdin = process.stdin;
const stdout = process.stdout;
const stderr = process.stderr;

function usage() {
    return 'USAGE:' + "\n" +
        '    gutenberg-post-parser [FLAGS] [INPUT]' + "\n\n" +
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
const output = gutenberg_post_parser.root(content);

switch (emit) {
    case 'debug':
        console.log(output);

        break;

    case 'json':
        stdout.write(JSON.stringify(output, null, 4));

        break;
}
