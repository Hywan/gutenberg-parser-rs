<?php

if ('GET' !== $_SERVER['REQUEST_METHOD']) {
    header('HTTP/1.1 405 Method Not Allowed');
    header('Content-Type: text/plain');

    echo 'Only `GET` is allowed.';

    exit;
}

function serve_html(string $file): void {
    header('Content-Type: text/html');

    echo file_get_contents($file);
}

function serve_javascript(string $file): void {
    header('Content-Type: application/javascript');

    echo file_get_contents($file);
}

function serve_wasm(string $file): void {
    header('Content-Type: application/wasm');

    echo file_get_contents($file);
}

switch ($_SERVER['REQUEST_URI']) {
    case '/':
    case '/index.html':
        serve_html(__DIR__ . '/index.html');

        break;

    case '/index.mjs':
    case '/gutenberg_post_parser.mjs':
        serve_javascript(__DIR__ . $_SERVER['REQUEST_URI']);

        break;

    case '/gutenberg_post_parser.wasm':
        $accepting = preg_split('/\s*,\s*/', $_SERVER['HTTP_ACCEPT_ENCODING']);
        $encoding = null;
        $file = $_SERVER['REQUEST_URI'];

        if (true === in_array('br', $accepting)) {
            $file .= '.br';
            $encoding = 'br';
        } elseif (true === in_array('gzip', $accepting)) {
            $file .= '.gz';
            $encoding = 'gzip';
        }

        if (!empty($encoding)) {
            header('Content-Encoding: ' . $encoding);
        }

        serve_wasm(__DIR__ . $file, $encoding);

        break;

    default:
        header('HTTP/1.1 404 Not Found');
        header('Content-Type: text/plain');

        echo 'File `', $_SERVER['REQUEST_URI'], '` not found.';

        exit;
}
