use std::collections::HashMap;

#[derive(Debug)]
pub struct PrefixTree<T> {
    tab: HashMap<(u32,u32,char),(u32,bool,Option<T>)>
}

impl<T: Copy+Clone> PrefixTree<T> {
    pub fn new(sorted_word_list_with_payload: &[(&str,T)]) -> PrefixTree<T> {
        let mut tab: HashMap<(u32,u32,char),(u32,bool,Option<T>)> = HashMap::new();
        let word_list_len = sorted_word_list_with_payload.len();
        for i in 0..word_list_len {
            let (ref word, ref payload) = sorted_word_list_with_payload[i];
            let char_vec: Vec<char> = word.chars().collect();
            let word_len = char_vec.len();
            
            let mut row_no: u32 = 0;
            for j in 0..word_len {
                let is_terminal = j + 1 == word_len;
                let ch = char_vec[j];
                let child = tab.entry((row_no as u32,j as u32,ch))
                    .or_insert((i as u32, is_terminal,
                                if is_terminal {
                                    Some(payload.clone())
                                } else {
                                    None
                                }));
                let &mut (_row_no, _, _) = child;
                row_no = _row_no;
            }            
        }
        PrefixTree{tab: tab}
    }

    pub fn seek(&self, key: &(u32,u32,char)) -> Option<&(u32,bool,Option<T>)> {
        self.tab.get(key)
    }
}
