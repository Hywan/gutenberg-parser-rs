package com.wordpress.gutenberg.parser;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;

public interface Parser extends Library {
    String JNA_LIBRARY_NAME = "gutenberg_post_parser";
    NativeLibrary JNA_NATIVE_LIB = NativeLibrary.getInstance(JNA_LIBRARY_NAME);

    Parser INSTANCE = (Parser) Native.loadLibrary(JNA_LIBRARY_NAME, Parser.class);

    NodeSet root(String input);
    void dropNodeSet(NodeSet nodeSet);
    void dropNode(RawNode node);
}
