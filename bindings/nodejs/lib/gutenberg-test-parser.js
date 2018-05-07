#!/usr/bin/env node

const parser = require('../native');

const stdin = process.stdin;
const stdout = process.stdout;
let input = '';

stdin.setEncoding('utf-8');
stdin.on(
    'readable',
    () => {
        let chunk;

        while (chunk = stdin.read()) {
            input += chunk;
        }
    }
);
stdin.on(
    'end',
    () => {
        stdout.write(
            JSON.stringify(
                parser.root(input)
            )
        );
    }
);
