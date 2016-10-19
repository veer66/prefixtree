use std::collections::HashMap;

#[derive(Debug)]
pub struct PrefixTree {
    tab: HashMap<(u32,u32,char),(u32,bool)>
}

impl PrefixTree {
    pub fn new(sorted_word_list: &[&str]) -> PrefixTree {
        let mut tab = HashMap::new();
        let word_list_len = sorted_word_list.len();
        for i in 0..word_list_len {
            let word = sorted_word_list[i].to_string();
            let char_vec: Vec<char> = word.chars().collect();
            let word_len = char_vec.len();
            
            let mut row_no: u32 = 0;
            for j in 0..word_len {
                let is_terminal = j + 1 == word_len;
                let ch = char_vec[j];    
                let r = tab.entry((row_no as u32,j as u32,ch))
                           .or_insert((i as u32, is_terminal));
                let &mut (_row_no, _) = r;
                row_no = _row_no;
            }            
        }
        PrefixTree{tab: tab}
    }

    pub fn seek(&self, key: &(u32,u32,char)) -> Option<(u32,bool)> {
        match self.tab.get(key) {
            Some(&(next_row, is_terminal)) => {
                Some((next_row, is_terminal))
            },
            None => None
        }
    }
}

#[test]
fn one_char_test() {
    let sorted_word_list = vec!["A"];
    let prefix_tree = PrefixTree::new(&sorted_word_list);
    assert!(prefix_tree.seek(&(0,0,'A')).unwrap() == (0,true));
    assert!(prefix_tree.seek(&(0,0,'B')).is_none());
}

#[test]
fn one_word_test() {
    let sorted_word_list = vec!["AB"];
    let prefix_tree = PrefixTree::new(&sorted_word_list);
    assert!(prefix_tree.seek(&(0,0,'A')).unwrap() == (0,false));
    assert!(prefix_tree.seek(&(0,1,'B')).unwrap() == (0,true));
}

#[test]
fn two_word_test() {
    let sorted_word_list = vec!["A", "AB"];
    let prefix_tree = PrefixTree::new(&sorted_word_list);
    assert!(prefix_tree.seek(&(0,0,'A')).unwrap() == (0,true));
    assert!(prefix_tree.seek(&(0,1,'B')).unwrap() == (1,true));
}

#[test]
fn three_word_test() {
    let sorted_word_list = vec!["A", "AB", "CXX"];
    let prefix_tree = PrefixTree::new(&sorted_word_list);
    assert!(prefix_tree.seek(&(0,0,'A')).unwrap() == (0,true));
    assert!(prefix_tree.seek(&(0,1,'B')).unwrap() == (1,true));
    assert!(prefix_tree.seek(&(0,0,'C')).unwrap() == (2,false));
    assert!(prefix_tree.seek(&(2,1,'X')).unwrap() == (2,false));
    assert!(prefix_tree.seek(&(2,2,'X')).unwrap() == (2,true));
}