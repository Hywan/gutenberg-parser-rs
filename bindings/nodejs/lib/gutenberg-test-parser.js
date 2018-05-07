#!/usr/bin/env node

const fs = require('fs');
const parser = require('../native');
const arguments = process.argv.slice(2);

function parseFromFile(file_input, file_output) {
    const input = fs.readFileSync(file_input, 'utf-8');
    const output = parser.root(input);

    for (let i in output) {
        const block = output[i];

        if (block['attrs']) {
            block['attrs'] = JSON.parse(block['attrs']);
        }
    }

    fs.writeFileSync(file_output, JSON.stringify(output));
}

parseFromFile(arguments[0], arguments[1]);
