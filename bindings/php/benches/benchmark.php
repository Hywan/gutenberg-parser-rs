<?php

declare(strict_types=1);

/**
 * @BeforeMethods({"init_content"})
 * @ParamProviders({"input_path"})
 * @Warmup(2)
 * @Revs(1000)
 * @Iterations(10)
 * @OutputTimeUnit("microseconds", precision=3)
 * @OutputMode("time")
 */
class Benches
{
    private $content = null;

    public function init_content(array $parameters = [])
    {
        $this->content = file_get_contents(__DIR__ . '/../../../tests/fixtures/' . $parameters['subject'] . '.html');
    }

    public function input_path(): array {
        return [
            ['subject' => 'autoclosing-block'],
            ['subject' => 'early-adopting-the-future'],
            ['subject' => 'gutenberg-demo'],
            ['subject' => 'moby-dick-parsed'],
            ['subject' => 'pygmalian-raw-html'],
            ['subject' => 'redesigning-chrome-desktop'],
            ['subject' => 'shortcode-shortcomings'],
            ['subject' => 'web-at-maximum-fps']
        ];
    }

    public function bench(array $parameters)
    {
        gutenberg_post_parse($this->content);
    }
}
