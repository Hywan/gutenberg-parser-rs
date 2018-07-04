#!/usr/bin/env node --experimental-modules
import { Gutenberg_Post_Parser } from '../../bin/gutenberg_post_parser.mjs';
import fs from 'fs';
import http from 'http';
import path from 'path';
import url from 'url';
import util from 'util';

const rss = {};

rss.start = process.memoryUsage().rss;

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
        this.blockName = name;
        this.attrs = attributes || {};
        this.innerBlocks = children;
    }
}

class Phrase {
    constructor(phrase) {
        this.innerHTML = phrase;
    }
}

let parser;

const server = http.createServer(
    (request, response) => {
        if ('POST' !== request.method) {
            response.writeHead(405, { 'Content-Type': 'text/plain' });
            response.end('Only `POST` is allowed.');
        }

        let input = '';

        request.on(
            'data',
            (data) => {
                input += data;
            }
        );

        request.on(
            'end',
            () => {
                rss.beforeParserInit = process.memoryUsage().rss;

                if (!parser) {
                    parser = new Gutenberg_Post_Parser_NodeJS(Block, Phrase, path.dirname(process.argv[1]) + '/../../bin/gutenberg_post_parser.wasm');
                }

                rss.afterParserInit = process.memoryUsage().rss;
                const timeStart = process.hrtime();

                parser.root(input).then(
                    (output) => {
                        const [timeEndSecond, timeEndNanosecond] = process.hrtime(timeStart);
                        rss.end = process.memoryUsage().rss;

                        response.write(
                            JSON.stringify(
                                {
                                    'parse': output,
                                    'us': timeEndSecond * 1e6 + timeEndNanosecond / 1000,
                                    'usAvg': timeEndSecond * 1e6 + timeEndNanosecond / 1000,
                                    'rss': rss
                                }
                            )
                        );
                        response.end();
                    }
                );
            }
        );

    }
);

const server_address = process.env.GUTENBERG_TEST_SERVER_ADDRESS;

if (!server_address) {
    console.log('The `GUTENBERG_TEST_SERVER_ADDRESS` environment variable is missing.');

    process.exit(1);
}

const parsed_server_address = url.parse('http://' + server_address);

server.listen(parsed_server_address.port, parsed_server_address.hostname);

console.log('Listening ' + server_address + '.');
