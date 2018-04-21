mod fs_tree;
mod json;
mod yaml;

extern crate clap;

use clap::{App, Arg};
use fs_tree::FSNode;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
enum ContentFormat {
    JSON,
    YAML
}

fn str_format_to_enum(format: Option<&str>) -> Option<ContentFormat> {
    if let Some(format) = format {
        return match format.to_lowercase().as_str() {
            "json" => Some(ContentFormat::JSON),
            "yaml" | "yml" => Some(ContentFormat::YAML),
            _ => None,
        }
     }
     None
}

fn format_from_file_name(path: &str) -> Option<ContentFormat> {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => str_format_to_enum(ext.to_str()),
        None => None,
    }
}

fn main() {
    let matches = App::new("fm_tree")
        .version("0.1")
        .author("Mykola Basov")
        .about("Format YAML/JSON struct to file system tree.")
        .arg(Arg::with_name("format")
             .help("Specify which format will be supplied to build tree. Required only for STDIN or files without proper extension.")
             .short("f")
             .long("format")
             .takes_value(true)
             .required(false))
        .arg(Arg::with_name("INPUT")
             .help("Path to file from which tree will be built. If no file then from STDIN.")
             .required(false)
             .index(1))
        .get_matches();

    let mut file_name: Option<&str> = None;
    let content = match matches.value_of("INPUT") {
        Some(file) => {
            file_name = Some(&file);
            fs::read_to_string(file).expect("Cannot read file.")
        },
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap();
            buf
        },
    };

    let format = match matches.value_of("format") {
        None => {
            if let Some(file_name) = file_name {
                match format_from_file_name(file_name) {
                    Some(format) => format,
                    None => {
                        panic!("Cannot derive format from file name.");
                    }
                }
            } else {
                panic!("You need to pass '--format' option.");
            }
        },
        format => {
            if let Some(format) = str_format_to_enum(format) {
                format
            } else {
                panic!("Specified format is not supported.");
            }
        },
    };

    let tree = match format {
        ContentFormat::JSON => FSNode::from_json(&content),
        ContentFormat::YAML => FSNode::from_yaml(&content),
    };
    println!("{}", tree);
}
