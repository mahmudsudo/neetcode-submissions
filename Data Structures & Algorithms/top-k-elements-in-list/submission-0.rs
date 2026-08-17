use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for el in nums {
            *map.entry(el).or_insert(0) += 1;
        }

        let mut res: Vec<(i32, i32)> = map.into_iter().collect();

        res.sort_by(|a, b| b.1.cmp(&a.1));

        res.into_iter()
            .take(k as usize)
            .map(|(num, _)| num)
            .collect()
    }
}