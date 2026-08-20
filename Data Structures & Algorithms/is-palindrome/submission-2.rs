impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();

        if s.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            while left < right && !s[left].is_ascii_alphanumeric() {
                left += 1;
            }

            while left < right && !s[right].is_ascii_alphanumeric() {
                right -= 1;
            }

            // They may have crossed while skipping characters
            if left >= right {
                break;
            }

            if s[left].to_ascii_lowercase() != s[right].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}