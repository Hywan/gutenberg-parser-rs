<?php

if ('POST' !== $_SERVER['REQUEST_METHOD']) {
    header('HTTP/1.1 405 Method Not Allowed');
    header('Content-Type: text/plain');

    echo 'Only `POST` is allowed.';

    exit;
}

$rss = [];
$rss['start'] =
    $rss['beforeParserInit'] =
    $rss['afterParserInit'] =
    memory_get_usage();

$input = file_get_contents('php://input');
$output = [];

function into_json(array $nodes, array &$output) {
    foreach ($nodes as $node) {
        if ($node instanceof Gutenberg_Parser_Block) {
            $children = [];

            if (null !== $node->children) {
                into_json($node->children, $children);
            }

            $output[] = [
                'blockName'   => $node->namespace . '/' . $node->name,
                'attrs'       => json_decode($node->attributes),
                'innerBlocks' => $children
            ];
        } elseif ($node instanceof Gutenberg_Parser_Phrase) {
            $output[] = [
                'innerHTML' => $node->content
            ];
        }
    }
}

$timeStart = microtime(true);

into_json(gutenberg_post_parse($input), $output);

$timeEnd = microtime(true) - $timeStart;
$rss['end'] = memory_get_usage();

header('Content-Type: application/json');
header('Access-Control-Allow-Origin: *');

echo json_encode([
    'parse' => $output,
    'us'    => $timeEnd,
    'usAvg' => $timeEnd,
    'rss'   => $rss
]);
