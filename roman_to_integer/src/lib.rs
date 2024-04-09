#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut acc = 0;
        let mut chars = s.chars().peekable();
        while let Some(ch) = chars.next() {
            let Some(peek) = chars.peek() else {
                return acc + char_to_num(ch);
            };

            let subtract_chars = match ch {
                'I' => ['V', 'X'],
                'X' => ['L', 'C'],
                'C' => ['D', 'M'],
                _ => ['\0', '\0'],
            };

            if subtract_chars.contains(peek) {
                acc += char_to_num(*peek) - char_to_num(ch);
                chars.next();
            } else {
                acc += char_to_num(ch);
            }
        }
        return acc;
    }
}

#[inline]
fn char_to_num(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => unreachable!(),
    }
}
