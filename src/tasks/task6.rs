/// 151. Reverse Words in a String
/// Given an input string s, reverse the order of the words.
///
/// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.
/// Return a string of the words in reverse order concatenated by a single space.
/// Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.
///
/// Example 1:
/// Input: s = "the sky is blue"
/// Output: "blue is sky the"

pub fn _reverse_words(s: String) -> String {
    let mut words = s
        .trim()
        .split(" ")
        .filter(|word| !word.is_empty())
        .collect::<Vec<&str>>();
    words.reverse();
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        println!("\n===================== ROUND 1 ===========================\n");
        let text = String::from("  hello world  ");
        let check = String::from("world hello");

        let result = _reverse_words(text);

        assert_eq!(result, check);
    }

    #[test]
    fn example2() {
        println!("\n===================== ROUND 2 ===========================\n");
        let text = String::from("  hello    world  ");
        let check = String::from("world hello");

        let result = _reverse_words(text);

        assert_eq!(result, check);
    }
}
