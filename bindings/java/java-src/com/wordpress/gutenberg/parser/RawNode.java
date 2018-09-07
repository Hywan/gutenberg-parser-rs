package com.wordpress.gutenberg.parser;

import com.sun.jna.Structure;
import java.io.Closeable;
import java.util.Arrays;
import java.util.List;

public class RawNode extends Structure implements Closeable {
    public static class ByReference extends RawNode implements Structure.ByReference { }

    public int nodeType;
    public String namespace;
    public String name;
    public String attributes;
    public String content;
    public NodeSet.ByReference children;

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "nodeType",
            "namespace",
            "name",
            "attributes",
            "content",
            "children"
        );
    }

    @Override
    public void close() {
        this.setAutoSynch(false);
        Parser.INSTANCE.dropNode(this);
    }
}
