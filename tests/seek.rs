extern crate prefixtree;
    
#[cfg(test)]
mod tests {
    use prefixtree::{PrefixTree, prefix_tree_from_str};

    #[test]
    fn one_char_test() {
        let sorted_word_list: Vec<(&str, i32)> = vec![("A", 1)];
        let prefix_tree = prefix_tree_from_str(&sorted_word_list[..]);
        assert!(prefix_tree.seek(&(0,0,'A')) == Some(&(0, true, Some(1))));
        assert!(prefix_tree.seek(&(0,0,'B')).is_none());
    }

    
    #[test]
    fn one_word_test() {
        let sorted_word_list = vec![("AB", 1)];
        let prefix_tree = prefix_tree_from_str(&sorted_word_list[..]);
        assert!(prefix_tree.seek(&(0,0,'A')) == Some(&(0,false,None)));
        assert!(prefix_tree.seek(&(0,1,'B')) == Some(&(0,true,Some(1))));
    }

    #[test]
    fn two_word_test() {
        let sorted_word_list = vec![("A",1), ("AB",2)];
        let prefix_tree = prefix_tree_from_str(&sorted_word_list[..]);
        assert!(prefix_tree.seek(&(0,0,'A')) == Some(&(0,true,Some(1))));
        assert!(prefix_tree.seek(&(0,1,'B')) == Some(&(1,true,Some(2))));
    }

    #[test]
    fn three_word_test() {
        let sorted_word_list = vec![("A",1), ("AB",2), ("CXX",3)];
        let prefix_tree = prefix_tree_from_str(&sorted_word_list[..]);
        assert!(prefix_tree.seek(&(0,0,'A')) == Some(&(0,true,Some(1))));
        assert!(prefix_tree.seek(&(0,1,'B')) == Some(&(1,true,Some(2))));
        assert!(prefix_tree.seek(&(0,0,'C')) == Some(&(2,false,None)));
        assert!(prefix_tree.seek(&(2,1,'X')) == Some(&(2,false,None)));
        assert!(prefix_tree.seek(&(2,2,'X')) == Some(&(2,true,Some(3))));
    }

    #[test]
    fn vec_of_vec_test() {
        let sorted_list = vec![
            (vec![1,2], 1),
            (vec![1,3], 2)
        ];
        let prefix_tree = PrefixTree::new(&sorted_list);
        assert!(prefix_tree.seek(&(0,0,1)) == Some(&(0,false,None)));
        assert!(prefix_tree.seek(&(0,1,2)) == Some(&(0,true,Some(1))));
        assert!(prefix_tree.seek(&(0,1,3)) == Some(&(1,true,Some(2))));
    }

    #[test]
    fn vec_of_str_vec_test() {
        let sorted_list = vec![
            (vec!["CAT", "DOG"], 1),
            (vec!["CAT", "ZOO"], 2)
        ];
        let prefix_tree = PrefixTree::new(&sorted_list);
        assert!(prefix_tree.seek(&(0,0,"CAT")) == Some(&(0,false,None)));
        assert!(prefix_tree.seek(&(0,1,"DOG")) == Some(&(0,true,Some(1))));
        assert!(prefix_tree.seek(&(0,1,"ZOO")) == Some(&(1,true,Some(2))));

    }
    
}
