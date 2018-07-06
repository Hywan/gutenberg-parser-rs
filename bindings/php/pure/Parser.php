<?php

namespace Gutenberg;

require __DIR__ . '/vendor/autoload.php';

use Hoa\Compiler;
use Hoa\File;
use Hoa\Visitor;

class Parser
{
    private static $parser = null;
    private static $visitor = null;

    public function __construct()
    {
        if (null === static::$parser) {
            static::$parser = Compiler\LLk\Llk::load(new File\Read(__DIR__ . '/Grammar.pp'));
        }

        if (null === static::$visitor) {
            static::$visitor = new IntoAST();
        }
    }

    public function parse(string $content)
    {
        return static::$visitor->visit(static::$parser->parse($content));
    }
}

class IntoAST implements Visitor\Visit
{
    public function visit(Visitor\Element $element, &$handle = null, $eldnah = null)
    {
        switch ($element->getId()) {
            case '#block_list':
                $output = [];

                foreach ($element->getChildren() as $child) {
                    $output[] = $child->accept($this, $handle, $eldnah);
                }

                return $output;

            case '#block_balanced':
                $block = new Block();

                $childN = 0;
                $numberOfChildren = $element->getChildrenNumber() - 1;

                $child = $element->getChild($childN);

                $fullname = $child->getValueValue();
                $namespace = 'core';
                $name = $fullname;

                if (false !== $position = strpos($fullname, '/')) {
                    $namespace = substr($fullname, 0, $position);
                    $name = substr($fullname, $position + 1);
                }

                $block->namespace = $namespace;
                $block->name = $name;

                if ($numberOfChildren <= $childN) {
                    return $block;
                }

                $childN++;
                $child = $element->getChild($childN);

                if ('token' === $child->getId()) {
                    $block->attributes = json_decode($child->getValueValue());

                    if ($numberOfChildren <= $childN) {
                        return $block;
                    }

                    $childN++;
                }

                $child = $element->getChild($childN);

                if ('#block_list' === $child->getId()) {
                    $block->children = $this->visit($child, $handle, $eldnah);
                }

                return $block;

            case '#block_void':
                $block = new Block();

                $child = $element->getChild(0);

                $fullname = $child->getValueValue();
                $namespace = 'core';
                $name = $fullname;

                if (false !== $position = strpos($fullname, '/')) {
                    $namespace = substr($fullname, 0, $position);
                    $name = substr($fullname, $position + 1);
                }

                $block->namespace = $namespace;
                $block->name = $name;

                if (2 > $element->getChildrenNumber()) {
                    return $block;
                }

                $child = $element->getChild(1);

                if ('token' === $child->getId()) {
                    $block->attributes = json_decode($child->getValueValue());
                }

                return $block;

            case '#phrase':
            case '#phrase_tail':
                $phrase = new Phrase();

                $phrase->content = $element->getChild(0)->getValueValue();

                return $phrase;
        }
    }
}

class Block
{
    public $namespace;
    public $name;
    public $attributes;
    public $children;
}

class Phrase
{
    public $content;
}
