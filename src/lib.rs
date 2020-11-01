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
//! assert_eq!(matcher.matches("store.apple.com").unwrap(), true);
//! ```

use hashbrown::HashMap;
use trust_dns_proto::error::ProtoResult;
use trust_dns_proto::rr::domain::IntoName;
use trust_dns_proto::rr::domain::Label;

#[derive(Debug, PartialEq, Clone)]
struct LevelNode {
    next_lvs: HashMap<Label, LevelNode>,
}

impl LevelNode {
    fn new() -> Self {
        Self {
            next_lvs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
/// Dmatcher matcher algorithm
pub struct Dmatcher {
    root: LevelNode,
}

impl Default for Dmatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Dmatcher {
    /// Create a matcher.
    pub fn new() -> Self {
        Self {
            root: LevelNode {
                next_lvs: HashMap::new(),
            },
        }
    }

    #[cfg(test)]
    fn get_root(&self) -> &LevelNode {
        &self.root
    }

    /// Pass in a string containing `\n` and get all domains inserted.
    pub fn insert_lines(&mut self, domain: String) -> ProtoResult<()> {
        let lvs: Vec<&str> = domain.split('\n').collect();
        for lv in lvs {
            self.insert(lv)?;
        }
        Ok(())
    }

    /// Pass in a domain and insert it into the matcher.
    pub fn insert<T: IntoName>(&mut self, domain: T) -> ProtoResult<()> {
        let lvs = T::into_name(domain)?;
        let lvs = lvs.iter().rev();
        let mut ptr = &mut self.root;
        for lv in lvs {
            ptr = ptr
                .next_lvs
                .entry(Label::from_raw_bytes(lv)?)
                .or_insert_with(LevelNode::new);
        }
        Ok(())
    }

    /// Match the domain against inserted domain rules. If `apple.com` is inserted, then `www.apple.com` and `stores.www.apple.com` is considered as matched while `apple.cn` is not.
    pub fn matches<T: IntoName>(&self, domain: T) -> ProtoResult<bool> {
        let lvs = T::into_name(domain)?;
        let lvs = lvs.iter().rev();
        let mut ptr = &self.root;
        for lv in lvs {
            if ptr.next_lvs.is_empty() {
                break;
            }
            ptr = match ptr.next_lvs.get(&Label::from_raw_bytes(lv)?) {
                Some(v) => v,
                None => return Ok(false),
            };
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::{Dmatcher, LevelNode};
    use hashbrown::HashMap;
    use trust_dns_proto::error::ProtoResult;
    use trust_dns_proto::rr::domain::Label;

    #[test]
    fn matches() -> ProtoResult<()> {
        let mut matcher = Dmatcher::new();
        matcher.insert("apple.com")?;
        matcher.insert("apple.cn")?;
        assert_eq!(matcher.matches("store.apple.com")?, true);
        assert_eq!(matcher.matches("baidu")?, false);
        assert_eq!(matcher.matches("你好.store.www.apple.com")?, true);
        Ok(())
    }

    #[test]
    fn insertion() -> ProtoResult<()> {
        let mut matcher = Dmatcher::new();
        matcher.insert("apple.com")?;
        matcher.insert("apple.cn")?;
        println!("{:?}", matcher.get_root());
        assert_eq!(
            matcher.get_root(),
            &LevelNode {
                next_lvs: [
                    (
                        Label::from_utf8("cn")?,
                        LevelNode {
                            next_lvs: [(
                                Label::from_utf8("apple")?,
                                LevelNode {
                                    next_lvs: []
                                        .iter()
                                        .cloned()
                                        .collect::<HashMap<Label, LevelNode>>()
                                }
                            )]
                            .iter()
                            .cloned()
                            .collect::<HashMap<Label, LevelNode>>()
                        }
                    ),
                    (
                        Label::from_utf8("com")?,
                        LevelNode {
                            next_lvs: [(
                                Label::from_utf8("apple")?,
                                LevelNode {
                                    next_lvs: []
                                        .iter()
                                        .cloned()
                                        .collect::<HashMap<Label, LevelNode>>()
                                }
                            )]
                            .iter()
                            .cloned()
                            .collect::<HashMap<Label, LevelNode>>()
                        }
                    )
                ]
                .iter()
                .cloned()
                .collect::<HashMap<Label, LevelNode>>()
            }
        );
        Ok(())
    }
}
