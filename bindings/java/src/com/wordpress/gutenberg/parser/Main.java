package com.wordpress.gutenberg.parser;

public class Main {
    public static void main(String[] arguments) {
        System.out.println("hello");

        NodeSet nodeSet = Parser.INSTANCE.root();

        for (Node node: nodeSet.getNodes()) {
            if (node instanceof Node.Block) {
                System.out.println(((Node.Block) node).name);
            } else if (node instanceof Node.Phrase) {
                System.out.println(((Node.Phrase) node).content);
            }
        }
    }
}
