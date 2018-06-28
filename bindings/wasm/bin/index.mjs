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

const parser = new Gutenberg_Post_Parser_NodeJS(Block, Phrase, './gutenberg_post_parser.wasm');

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
        parser.root(input).then(
            (output) => {
                stdout.write(JSON.stringify(output, null, 2));
            }
        );
    }
);
