use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

#[derive(Debug, Clone)]
pub struct PrefixTree<Elem,Payload> where Elem: Hash + Eq + Clone {
    tab: HashMap<(u32,u32,Elem),(u32,bool,Option<Payload>)>
}

impl<Elem: Hash+Eq+Clone, Payload: Clone> PrefixTree<Elem, Payload> {
    
    pub fn new(sorted_word_list_with_payload: &[(Vec<Elem>, Payload)])
               -> PrefixTree<Elem, Payload>
    {
        let mut tab: HashMap<(u32,u32,Elem),(u32,bool,Option<Payload>)> = HashMap::new();
        let word_list_len = sorted_word_list_with_payload.len();
        for i in 0..word_list_len {
            let (ref item, ref payload) = sorted_word_list_with_payload[i];
            let item_len = item.len();
            
            let mut row_no: u32 = 0;
            let mut j = 0;
            
            for elem in item.into_iter() {
                let elem = elem.clone();
                let is_terminal = j + 1 == item_len;
                let child = tab.entry((row_no as u32,j as u32, elem))
                    .or_insert((i as u32, is_terminal,
                                if is_terminal {
                                    Some(payload.clone())
                                } else {
                                    None
                                }));
                let &mut (_row_no, _, _) = child;
                row_no = _row_no;
                j += 1;
            }            
        }
        PrefixTree{tab: tab}
    }

    pub fn seek(&self, key: &(u32,u32,Elem)) -> Option<&(u32,bool,Option<Payload>)> {
        self.tab.get(key)
    }
}

pub fn prefix_tree_from_str<P:Clone>(sorted_word_list_with_payload: &[(&str, P)])
                                     -> PrefixTree<char, P>
{
    let sorted_word_list_with_payload: Vec<(Vec<char>,P)> =
        sorted_word_list_with_payload
        .into_iter()
        .map(|&(ref item, ref payload)|
             (item.chars().collect::<Vec<char>>(), payload.clone()))
        .collect();
    PrefixTree::new(&sorted_word_list_with_payload[..])
}
