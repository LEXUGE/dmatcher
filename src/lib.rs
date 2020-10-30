#![deny(missing_docs)]
// Documentation
//! This is a simple domain matching algorithm to match domains against a set of user-defined domain rules.
//!
//! Features:
//!
//! -  Super fast (150 ns per match for a 73300+ domain rule set)
//! -  No dependencies
//!
//! # Getting Started
//!
//! ```
//! use dmatcher::Dmatcher;
//! let mut matcher = Dmatcher::new();
//! matcher.insert("apple.com");
//! assert_eq!(matcher.matches("store.apple.com"), true);
//! ```

use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct LevelNode<'a> {
    name: Option<&'a str>,
    next_lvs: HashMap<&'a str, LevelNode<'a>>,
}

impl<'a> LevelNode<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name: Some(name),
            next_lvs: HashMap::new(),
        }
    }
}

#[derive(Debug)]
/// Dmatcher matcher algorithm
pub struct Dmatcher<'a> {
    root: LevelNode<'a>,
}

impl<'a> Default for Dmatcher<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Dmatcher<'a> {
    /// Create a matcher.
    pub fn new() -> Self {
        Self {
            root: LevelNode {
                name: None,
                next_lvs: HashMap::new(),
            },
        }
    }

    #[cfg(test)]
    fn get_root(&self) -> &LevelNode {
        &self.root
    }

    /// Pass in a string containing `\n` and get all domains inserted.
    pub fn insert_lines(&mut self, domain: &'a str) {
        let lvs: Vec<&str> = domain.split('\n').collect();
        for lv in lvs {
            self.insert(lv);
        }
    }

    /// Pass in a domain and insert it in the matcher.
    pub fn insert(&mut self, domain: &'a str) {
        let mut lvs: Vec<&str> = domain.split('.').collect();
        lvs.reverse();
        let mut ptr = &mut self.root;
        for lv in lvs {
            if lv == "" {
                // We should not include sub-levels like ""
                continue;
            }
            ptr = ptr.next_lvs.entry(lv).or_insert_with(|| LevelNode::new(lv));
        }
    }

    /// Match the domain against inserted domain rules. If `apple.com` is inserted, then `www.apple.com` and `stores.www.apple.com` is considered as matched while `apple.cn` is not.
    pub fn matches(&self, domain: &str) -> bool {
        let mut lvs: Vec<&str> = domain.split('.').collect();
        lvs.reverse();
        let mut ptr = &self.root;
        for lv in lvs {
            if lv == "" {
                // We should not include sub-levels like ""
                continue;
            }
            if ptr.next_lvs.is_empty() {
                break;
            }
            ptr = match ptr.next_lvs.get(lv) {
                Some(v) => v,
                None => return false,
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Dmatcher, LevelNode};
    use hashbrown::HashMap;

    #[test]
    fn matches() {
        let mut matcher = Dmatcher::new();
        matcher.insert("apple.com");
        matcher.insert("apple.cn");
        assert_eq!(matcher.matches("store.apple.com"), true);
        assert_eq!(matcher.matches("baidu"), false);
        assert_eq!(matcher.matches("你好.store.www.apple.com"), true);
    }

    #[test]
    fn insertion() {
        let mut matcher = Dmatcher::new();
        matcher.insert("apple.com");
        matcher.insert("apple.cn");
        println!("{:?}", matcher.get_root());
        assert_eq!(
            matcher.get_root(),
            &LevelNode {
                name: None,
                next_lvs: [
                    (
                        "cn",
                        LevelNode {
                            name: Some("cn"),
                            next_lvs: [(
                                "apple",
                                LevelNode {
                                    name: Some("apple"),
                                    next_lvs: []
                                        .iter()
                                        .cloned()
                                        .collect::<HashMap<&str, LevelNode>>()
                                }
                            )]
                            .iter()
                            .cloned()
                            .collect::<HashMap<&str, LevelNode>>()
                        }
                    ),
                    (
                        "com",
                        LevelNode {
                            name: Some("com"),
                            next_lvs: [(
                                "apple",
                                LevelNode {
                                    name: Some("apple"),
                                    next_lvs: []
                                        .iter()
                                        .cloned()
                                        .collect::<HashMap<&str, LevelNode>>()
                                }
                            )]
                            .iter()
                            .cloned()
                            .collect::<HashMap<&str, LevelNode>>()
                        }
                    )
                ]
                .iter()
                .cloned()
                .collect::<HashMap<&str, LevelNode>>()
            }
        );
    }
}
