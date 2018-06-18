package com.wordpress.gutenberg.parser;

import com.sun.jna.Structure;
import java.io.Closeable;
import java.util.Arrays;
import java.util.List;
import java.util.Vector;

public class NodeSet extends Structure implements Closeable {
    public static class ByReference extends NodeSet implements Structure.ByReference { }

    public RawNode.ByReference nodes;
    public int numberOfNodes;

    public Vector<Node> getNodes() {
        Vector<Node> result = new Vector();

        for (RawNode rawNode: (RawNode[]) this.nodes.toArray(this.numberOfNodes)) {
            if (0 == rawNode.nodeType) {
                Node.Block block = new Node.Block();
                block.namespace = rawNode.namespace;
                block.name = rawNode.name;
                block.attributes = rawNode.attributes;

                result.add(block);
            } else if (1 == rawNode.nodeType) {
                Node.Phrase phrase = new Node.Phrase();
                phrase.content = rawNode.content;

                result.add(phrase);
            }
        }

        return result;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "nodes",
            "numberOfNodes"
        );
    }

    @Override
    public void close() {
        this.setAutoSynch(false);
        Parser.INSTANCE.dropNodeSet(this);
    }
}
