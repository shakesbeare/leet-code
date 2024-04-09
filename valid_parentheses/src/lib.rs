pub struct Solution;

#[derive(PartialEq, Eq)]
enum Delimiter {
    Paren,
    Bracket,
    Curly
}

impl From<char> for Delimiter {
    fn from(value: char) -> Self {
        match value {
            '(' | ')' => Delimiter::Paren,
            '[' | ']' => Delimiter::Bracket,
            '{' | '}' => Delimiter::Curly,
            _ => panic!("Invalid delimiter")
        }
    }
}

impl Solution {
    fn is_valid(s: String) -> bool {
        let mut p_stack: Vec<Delimiter> = Vec::with_capacity(s.len());

        for c in s.chars() {
            if ['(', '[', '{'].contains(&c) {
                p_stack.push(c.into());
            } else if let Some(popped) = p_stack.pop() {
                if popped != c.into() {
                    return false;
                }
            } else {
                return false;
            }
        }
        return p_stack.is_empty()
    }
}
