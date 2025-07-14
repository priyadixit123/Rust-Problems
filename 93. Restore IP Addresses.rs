https://leetcode.com/problems/restore-ip-addresses/description/

SOL:

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        Self::backtrack(&s, 0, &mut path, &mut result);
        result
    }

    fn backtrack(s: &str, start: usize, path: &mut Vec<String>, result: &mut Vec<String>) {
        if path.len() == 4 {
            if start == s.len() {
                result.push(path.join("."));
            }
            return;
        }

        for len in 1..=3 {
            if start + len > s.len() {
                break;
            }
            let segment = &s[start..start + len];

            // Skip invalid segments
            if !Self::is_valid_segment(segment) {
                continue;
            }

            path.push(segment.to_string());
            Self::backtrack(s, start + len, path, result);
            path.pop();
        }
    }

    fn is_valid_segment(seg: &str) -> bool {
        if seg.len() > 1 && seg.starts_with('0') {
            return false;
        }
        match seg.parse::<u8>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
