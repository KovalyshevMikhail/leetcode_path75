/// You are given two strings word1 and word2.
/// Merge the strings by adding letters in alternating order, starting with word1.
/// If a string is longer than the other, append the additional letters onto the end of the merged string.
///
/// Return the merged string.
///
/// Example 1:
///
/// Input: word1 = "abc", word2 = "pqr"
/// Output: "apbqcr"
/// Explanation: The merged string will be merged as so:
/// word1:  a   b   c
/// word2:    p   q   r
/// merged: a p b q c r
///
pub fn _merge_alternately(word1: String, word2: String) -> String {
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();
    let mut out = String::new();
    let mut is_end = false;

    while !is_end {
        match (chars1.next(), chars2.next()) {
            (None, None) => {
                is_end = true;
                continue;
            }
            (Some(ch1), Some(ch2)) => {
                out.push(ch1);
                out.push(ch2);
            }
            (Some(ch1), None) => {
                out.push(ch1);
            }
            (None, Some(ch2)) => {
                out.push(ch2);
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        let check = String::from("apbqcr");

        let result = _merge_alternately(word1, word2);
        assert_eq!(result, check);
    }

    #[test]
    fn example2() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        let check = String::from("apbqrs");

        let result = _merge_alternately(word1, word2);
        assert_eq!(result, check);
    }

    #[test]
    fn example3() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        let check = String::from("apbqcd");

        let result = _merge_alternately(word1, word2);
        assert_eq!(result, check);
    }
}
