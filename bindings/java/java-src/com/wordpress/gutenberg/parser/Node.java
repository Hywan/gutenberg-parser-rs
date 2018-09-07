package com.wordpress.gutenberg.parser;

public interface Node {
    public class Block implements Node {
        public String namespace;
        public String name;
        public String attributes;
    }

    public class Phrase implements Node {
        public String content;
    }
}
