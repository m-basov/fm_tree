extern crate serde_yaml;

use self::serde_yaml::Value;
use fs_tree::FSNode;

fn collect(yaml: &[Value]) -> Vec<FSNode> {
    let mut nodes = vec![];

    for val in yaml {
        if val.is_string() {
            let file_name = val.as_str().unwrap().to_owned();
            nodes.push(FSNode::new(file_name));
        } else if val.is_mapping() {
            for (key, item) in val.as_mapping().unwrap() {
                let key = key.as_str().unwrap().to_owned();
                let node = if item.is_string() {
                    FSNode::new(key)
                } else {
                    let item = item.as_sequence()
                        .expect("Values may be only strings or sequences");
                    FSNode {
                        name: key,
                        children: collect(item),
                    }
                };
                nodes.push(node);
            }
        }
    }

    nodes
}

pub fn parse(yaml: &str) -> Vec<FSNode> {
    let raw: Vec<Value> = serde_yaml::from_str(yaml).expect("YAML is invalid");
    collect(&raw)
}
