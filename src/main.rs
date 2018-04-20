mod fs_tree;

extern crate clap;

use std::fs;
use std::io;
use std::io::Read;
use clap::{App,Arg};
use fs_tree::FSNode;

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

    if let Some(file) = matches.value_of("INPUT") {
        let content = fs::read_to_string(file).expect("Cannot read file.");
        println!("{}", FSNode::from_yaml(&content));
    } else {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).unwrap();
        println!("{}", FSNode::from_yaml(&content));
    }
}
