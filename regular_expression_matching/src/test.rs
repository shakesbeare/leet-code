mod tests {
    #[allow(unused_imports)]
    use crate::Solution;

    #[test]
    fn basic() {
        assert!(Solution::is_match("aabb".to_string(), "aabb".to_string()));
    }

    #[test]
    fn no() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn basic_repeat() {
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn any_repeat() {
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn not_greedy() {
        assert!(Solution::is_match("aaa".to_string(), "a*a".to_string()));
    }

    #[test]
    fn no_match_with_wildcard_repeat() {
        assert!(!Solution::is_match("ab".to_string(), ".*c".to_string()));
    }

    #[test]
    fn big() {
        assert!(Solution::is_match(
            "aaa".to_string(),
            "ab*a*c*a".to_string()
        ));
    }

    #[test]
    fn asterisks_allow_zero() {
        assert!(Solution::is_match("a".to_string(), "ab*".to_string()));
    }

    #[test]
    fn enormous() {
        assert!(Solution::is_match(
            "aabcbcbcaccbcaabc".to_string(),
            ".*a*aa*.*b*.c*.*a*".to_string()
        ));
    }
}
