extern crate serde_json;

use self::serde_json::{Map,Value};
use fs_tree::FSNode;

fn collect(raw: &Map<String, Value>) -> Vec<FSNode> {
    let mut nodes = vec![];

    for (key, val) in raw {
        if val.is_object() {
            let mut node = FSNode::new(key.to_owned());
            node.children = collect(val.as_object().unwrap());
            nodes.push(node);
        } else {
            let node = FSNode::new(key.to_owned());
            nodes.push(node);
        }
    }

    nodes
}

pub fn parse(json: &str) -> Vec<FSNode> {
    let raw: Value = serde_json::from_str(json).expect("JSON is invalid");
    let root = raw.as_object().expect("JSON root must be an object.");
    collect(&root)
}
