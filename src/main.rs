extern crate gutenberg_post_parser;
extern crate clap; 

use gutenberg_post_parser::{root, ast::Node};
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, prelude::*};
use std::str;

macro_rules! to_str (
    ($slice:expr) => (
        unsafe { str::from_utf8_unchecked($slice) }
    )
);
 
fn main() { 
    let matches = App::new("gutenberg-post-parser")
       .version(env!("CARGO_PKG_VERSION"))
       .about("Parse Gutenberg posts!")
       .author("Ivan Enderlin")
        .arg(
            Arg::with_name("INPUT")
                .help("File containing the input (if absent, read `stdin`).")
                .required(false)
                .index(1)
        )
       .get_matches(); 

    let mut content = String::new();

    match matches.value_of("INPUT") {
        Some(file_name) => {
            let mut file = File::open(file_name).unwrap();
            file.read_to_string(&mut content).unwrap();
        },

        None => {
            let stdin = io::stdin();
            stdin.lock().read_to_string(&mut content).unwrap();
        }
    }

    match root(content.as_bytes()) {
        Ok((_remaining, nodes)) => {
            let mut stdout = io::stdout();
            let mut lock = stdout.lock();

            serialize_nodes_to_json(lock, nodes);
        },

        Err(_) => {
            println!("Failed to parser the content.");
        }
    }
}

fn serialize_nodes_to_json<W: Write>(mut writer: W, nodes: Vec<Node>) {
    writer.write_all(&b"["[..]).unwrap();

    for (index, node) in nodes.iter().enumerate() {
        if 0 != index {
            writer.write_all(&b","[..]).unwrap();
        }

        serialize_node_to_json(&mut writer, node);
    }

    writer.write_all(&b"]"[..]).unwrap();
}

fn serialize_node_to_json<W: Write>(writer: &mut W, node: &Node) {
    match node {
        Node::Block { name, attributes, children } => {
            writer.write_all(&b"{"[..]).unwrap();

            write!(
                writer,
                "\"blockName\":\"{0}/{1}\",\"attrs\":{2}",
                to_str!(name.0),
                to_str!(name.1),
                match attributes {
                    Some(attributes) => to_str!(attributes),
                    None => "null"
                }
            ).unwrap();

            let mut blocks = vec![];
            let mut phrases = vec![];

            for child in children {
                match child {
                    Node::Block { .. } => blocks.push(child),
                    Node::Phrase(phrase) => phrases.push(phrase)
                }
            }

            writer.write_all(&b",\"innerBlocks\":["[..]).unwrap();

            for (index, block) in blocks.iter().enumerate() {
                if 0 != index {
                    writer.write_all(&b","[..]).unwrap();
                }

                serialize_node_to_json(writer, block);
            }

            writer.write_all(&b"],\"innerHTML\":\""[..]).unwrap();

            for phrase in phrases {
                writer.write_all(escape_json_literal(to_str!(phrase)).as_bytes()).unwrap();
            }

            writer.write_all(&b"\"}"[..]).unwrap();
        },

        Node::Phrase(phrase) => {
            write!(
                writer,
                "{{\"attrs\":{{}},\"innerHTML\":\"{0}\"}}",
                escape_json_literal(to_str!(phrase))
            ).unwrap();
        }
    }
}

fn escape_json_literal(str: &str) -> String {
    str
        .replace("\n", "\\n")
        .replace("\t", "\\t")
        .replace("\"", "\\\"")
}
