use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
      let mut s_sorted =  s.chars().collect::<Vec<_>>() ;
      let mut t_sorted = t.chars().collect::<Vec<_>>();

      s_sorted.sort();
      t_sorted.sort();
      s_sorted== t_sorted
    }
}
