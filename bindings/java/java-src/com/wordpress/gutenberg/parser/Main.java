package com.wordpress.gutenberg.parser;

public class Main {
    public static void main(String[] arguments) {
        NodeSet.ByReference nodeSet = Parser.INSTANCE.parse("<!-- wp:foo /-->hello");

        for (Node node: nodeSet.getNodes()) {
            if (node instanceof Node.Block) {
                System.out.println(((Node.Block) node).namespace);
                System.out.println(((Node.Block) node).name);
            } else if (node instanceof Node.Phrase) {
                System.out.println(((Node.Phrase) node).content);
            }
        }
    }
}
