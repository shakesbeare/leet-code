struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        'outer: for i in 0..strs[0].len() {
            let Some(ch) = strs[0].chars().nth(i) else {
                return prefix;
            };
            for string in &strs {
                let Some(c) = string.chars().nth(i) else {
                    return prefix;
                };
                if c != ch {
                    break 'outer;
                }
            }
            prefix.push(ch);
        }
        prefix
    }
}
