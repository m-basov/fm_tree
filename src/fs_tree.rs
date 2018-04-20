extern crate serde;
extern crate serde_yaml;

const CHAR_VER_PIPE: &str = "│   ";
const CHAR_T_PIPE: &str = "├── ";
const CHAR_ANG_PIPE: &str = "└── ";
const CHAR_EMPTY_PIPE: &str = "    ";

use std::fmt;
use std::fmt::Write;

#[derive(Debug)]
pub struct FSNode {
    name: String,
    children: Vec<FSNode>,
}
impl fmt::Display for FSNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        self.print(&mut result, 0, false, 0);
        write!(f, "{}", result)
    }
}

impl FSNode {
    pub fn new(name: String) -> FSNode {
        FSNode {
            name: name,
            children: vec![],
        }
    }

    fn print(&self, acc: &mut String, level: u32, is_last: bool, last_streak: u32) {
        let mut base = String::new();
        if level > 0 {
            for _ in 1..(level - last_streak) {
                base.push_str(CHAR_VER_PIPE);
            }
            for _ in 0..last_streak {
                base.push_str(CHAR_EMPTY_PIPE);
            }
            if is_last {
                base.push_str(CHAR_ANG_PIPE);
            } else {
                base.push_str(CHAR_T_PIPE);
            }
            write!(acc, "\n").unwrap();
        }

        write!(acc, "{}{}", base, self.name).unwrap();

        let length = self.children.len();
        for (idx, child) in (&self.children).into_iter().enumerate() {
            let next_is_last = idx == length - 1;
            let last_streak = if is_last && next_is_last {
                last_streak + 1
            } else {
                0
            };
            child.print(acc, level + 1, next_is_last, last_streak);
        }
    }

    pub fn from_yaml(yaml: &str) -> FSNode {
        let raw: Vec<serde_yaml::Value> = serde_yaml::from_str(yaml).expect("YAML is invalid");
        FSNode {
            name: ".".to_owned(),
            children: collect_from_yaml(&raw),
        }
    }
}

fn collect_from_yaml(yaml: &[serde_yaml::Value]) -> Vec<FSNode> {
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
                        children: collect_from_yaml(item),
                    }
                };
                nodes.push(node);
            }
        }
    }

    nodes
}
