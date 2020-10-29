use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct LevelNode<'a> {
    name: Option<&'a str>,
    next_lvs: HashMap<&'a str, LevelNode<'a>>,
}

impl<'a> LevelNode<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: Some(name),
            next_lvs: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Dmatcher<'a> {
    root: LevelNode<'a>,
}

impl<'a> Dmatcher<'a> {
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

    pub fn insert_lines(&mut self, domain: &'a str) {
        let lvs: Vec<&str> = domain.split('\n').collect();
        for lv in lvs {
            self.insert(lv);
        }
    }

    pub fn insert(&mut self, domain: &'a str) {
        let mut lvs: Vec<&str> = domain.split('.').collect();
        lvs.reverse();
        let mut ptr = &mut self.root;
        for lv in lvs {
            if lv == "" {
                // We should not include sub-levels like ""
                continue;
            }
            ptr = ptr.next_lvs.entry(lv).or_insert(LevelNode::new(lv));
        }
    }

    pub fn contains(&self, domain: &str) -> bool {
        let mut lvs: Vec<&str> = domain.split('.').collect();
        lvs.reverse();
        let mut ptr = &self.root;
        for lv in lvs {
            if ptr.next_lvs.len() == 0 {
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
    use std::collections::HashMap;

    #[test]
    fn contains() {
        let mut matcher = Dmatcher::new();
        matcher.insert("apple.com");
        matcher.insert("apple.cn");
        assert_eq!(matcher.contains("store.apple.com"), true);
        assert_eq!(matcher.contains("baidu"), false);
        assert_eq!(matcher.contains("你好.store.www.apple.com"), true);
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
