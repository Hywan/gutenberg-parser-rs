import process from 'process';
import fs from 'fs';
import path from  'path';
import util from 'util';
import { Gutenberg_Post_Parser } from '../bin/gutenberg_post_parser.mjs';
import Benchmark from 'benchmark';

const __dirname = path.dirname(process.argv[1]);

class Gutenberg_Post_Parser_NodeJS extends Gutenberg_Post_Parser {
    constructor(Block, Phrase, wasmURL) {
        super(Block, Phrase, wasmURL, new util.TextEncoder());
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

const parser = new Gutenberg_Post_Parser_NodeJS(Block, Phrase, __dirname + '/../bin/gutenberg_post_parser.wasm');

const suite = new Benchmark.Suite;

function bench(suite, name, file) {
    const content = fs.readFileSync(file, { encoding: 'utf-8' });

    suite.add(
        name,
        (deferred) => {
            parser.root(content).then((output) => { deferred.resolve() });
        },
        {
            defer: true
        }
    );
}

bench(suite, 'autoclosing_block', __dirname + '/../../../tests/fixtures/autoclosing-block.html');
bench(suite, 'early_adopting_the_future', __dirname + '/../../../tests/fixtures/early-adopting-the-future.html');
bench(suite, 'gutenberg_demo', __dirname + '/../../../tests/fixtures/gutenberg-demo.html');
bench(suite, 'moby_dick_parsed', __dirname + '/../../../tests/fixtures/moby-dick-parsed.html');
bench(suite, 'pygmalian_raw_html', __dirname + '/../../../tests/fixtures/pygmalian-raw-html.html');
bench(suite, 'redesigning_chrome_desktop', __dirname + '/../../../tests/fixtures/redesigning-chrome-desktop.html');
bench(suite, 'shortcode_shortcomings', __dirname + '/../../../tests/fixtures/shortcode-shortcomings.html');
bench(suite, 'web_at_maximum_fps', __dirname + '/../../../tests/fixtures/web-at-maximum-fps.html');

suite
    .on('cycle', (event) => {
        const bench = event.target;

        let output = bench.name + ' x ' +
            Benchmark.formatNumber(bench.hz.toFixed(0)) + ' ops/sec ' +
            '(' + (bench.stats.mean / 1e-3).toFixed(3) + 'ms) ' +
            '±' + bench.stats.rme.toFixed(2) + '% ' +
            '(' + bench.stats.sample.length + ' runs sampled)';
        
        process.stdout.write(output + '\n');

    })
    .on('complete', () => {
        let average = 0;

        suite.forEach(
            (bench) => {
                average += bench.stats.mean;
            }
        );

        average /= suite.length;

        process.stdout.write(
            '\nMean is ' + (average / 1e-3).toFixed(3) + 'ms.\n'
        );
    })
    .run();