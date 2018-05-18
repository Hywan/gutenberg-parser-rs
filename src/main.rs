extern crate gutenberg_post_parser;
#[macro_use] extern crate failure;
extern crate clap; 
extern crate nom;

use gutenberg_post_parser::{root, ast::Node, Input};
use failure::{Error, ResultExt};
use clap::{App, Arg};
use std::fs;
use std::io::{self, prelude::*};
use std::str;

macro_rules! to_str (
    ($slice:expr) => (
        unsafe { str::from_utf8_unchecked($slice) }
    )
);
 
fn main() -> Result<(), Error> {
    let matches =
        App::new("gutenberg-post-parser")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Parse Gutenberg posts!")
            .author("Ivan Enderlin")
            .arg(
                Arg::with_name("emit-json")
                    .help("Compile the AST into JSON (default).")
                    .short("j")
                    .long("emit-json")
            )
            .arg(
                Arg::with_name("emit-debug")
                    .help("Compile the AST into Rust debug format.")
                    .short("d")
                    .long("emit-debug")
            )
            .arg(
                Arg::with_name("INPUT")
                    .help("File containing the input (if absent, read `stdin`).")
                    .required(false)
                    .index(1)
            )
            .get_matches();

    let mut content;

    match matches.value_of("INPUT") {
        Some(file_name) => {
            content = fs::read_to_string(file_name).context("Cannot open or read the given file to parse.")?;
        },

        None => {
            let stdin = io::stdin();
            content = String::new();
            stdin.lock().read_to_string(&mut content).context("Cannot read from `stdin`.")?;
        }
    }

    match root(content.as_bytes()) {
        Ok((remaining, nodes)) => {
            if matches.is_present("emit-debug") {
                let debug: nom::IResult<Input, Vec<Node>> = Ok((remaining, nodes));

                print!("{:?}", debug);
            } else {
                let mut stdout = io::stdout();
                let mut lock = stdout.lock();

                serialize_nodes_to_json(lock, nodes).context("Failed to serialize parser output to JSON.")?;
            }
        },

        Err(_) => {
            return Err(format_err!("Failed to parse the datum."));
        }
    }

    Ok(())
}

fn serialize_nodes_to_json<W: Write>(mut writer: W, nodes: Vec<Node>) -> Result<(), Error> {
    writer.write_all(&b"["[..])?;

    for (index, node) in nodes.iter().enumerate() {
        if 0 != index {
            writer.write_all(&b","[..])?;
        }

        serialize_node_to_json(&mut writer, node)?;
    }

    writer.write_all(&b"]"[..])?;

    Ok(())
}

fn serialize_node_to_json<W: Write>(writer: &mut W, node: &Node) -> Result<(), Error> {
    match node {
        Node::Block { name, attributes, children } => {
            writer.write_all(&b"{"[..])?;

            write!(
                writer,
                "\"blockName\":\"{0}/{1}\",\"attrs\":{2}",
                to_str!(name.0),
                to_str!(name.1),
                match attributes {
                    Some(attributes) => to_str!(attributes),
                    None => "null"
                }
            )?;

            let mut blocks = vec![];
            let mut phrases = vec![];

            for child in children {
                match child {
                    Node::Block { .. } => blocks.push(child),
                    Node::Phrase(phrase) => phrases.push(phrase)
                }
            }

            writer.write_all(&b",\"innerBlocks\":["[..])?;

            for (index, block) in blocks.iter().enumerate() {
                if 0 != index {
                    writer.write_all(&b","[..])?;
                }

                serialize_node_to_json(writer, block)?;
            }

            writer.write_all(&b"],\"innerHTML\":\""[..])?;

            for phrase in phrases {
                writer.write_all(escape_json_literal(to_str!(phrase)).as_bytes())?;
            }

            writer.write_all(&b"\"}"[..])?;
        },

        Node::Phrase(phrase) => {
            write!(
                writer,
                "{{\"attrs\":{{}},\"innerHTML\":\"{0}\"}}",
                escape_json_literal(to_str!(phrase))
            )?;
        }
    }

    Ok(())
}

fn escape_json_literal(str: &str) -> String {
    str
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\"", "\\\"")
}
