extern crate serde;
extern crate serde_yaml;

const CHAR_VER_PIPE: &str = "│   ";
const CHAR_T_PIPE: &str = "├── ";
const CHAR_ANG_PIPE: &str = "└── ";
const CHAR_EMPTY_PIPE: &str = "    ";

use std::fmt;
use std::fmt::Write;
use yaml;
use json;

#[derive(Debug)]
pub struct FSNode {
    pub name: String,
    pub children: Vec<FSNode>,
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
        FSNode {
            name: ".".to_owned(),
            children: yaml::parse(yaml),
        }
    }

    pub fn from_json(json: &str) -> FSNode {
        FSNode {
            name: ".".to_owned(),
            children: json::parse(json),
        }
    }
}
