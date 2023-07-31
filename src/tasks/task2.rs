/// greatest-common-divisor-of-strings
///
/// For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
///
/// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
///
/// Example 1:
///
/// Input: str1 = "ABCABC", str2 = "ABC"
/// Output: "ABC"

pub fn _gcd_of_strings(str1: String, str2: String) -> String {
    fn complex_check<'a>(
        str1: &'a str,
        str2: &'a str,
        separator: &'a str,
    ) -> (bool, Option<&'a str>) {
        let check_1 = check(str1, separator);
        let check_2 = check(str2, separator);

        let separator_add_1 = check_1.1.unwrap_or("");
        let separator_add_2 = check_2.1.unwrap_or("");

        let returns = if separator_add_1 == separator_add_2 && !separator_add_1.is_empty() {
            (check_1.0 && check_2.0, Some(separator_add_1))
        } else {
            (check_1.0 && check_2.0, None)
        };

        returns
    }

    fn check<'a>(str: &'a str, separator: &'a str) -> (bool, Option<&'a str>) {
        let split = str.split(separator).collect::<Vec<&str>>();

        let check_size = split.len() > 1;
        let check_empty = split.iter().all(|part| part.is_empty());

        let part: Option<&str> = {
            let first = split.first().unwrap();
            let mut iter = split.iter().skip(1);

            match iter.next() {
                Some(iter_first) => {
                    if first.is_empty() && iter.all(|part| part == iter_first) {
                        Some(iter_first)
                    } else {
                        None
                    }
                }
                None => None,
            }
        };

        (check_size && check_empty, part)
    }

    let main_check = check(str1.as_str(), str2.as_str());

    if main_check.0 {
        str2
    } else {
        let mut separator = String::new();
        let mut variant = String::new();
        let mut chars = str2.chars();
        let mut is_end = false;

        while !is_end {
            match chars.next() {
                Some(ch) => {
                    separator.push(ch);
                    let check = complex_check(str1.as_str(), str2.as_str(), separator.as_str());

                    match check.1 {
                        Some(part) => {
                            let part = String::from(part);

                            separator.push_str(part.as_str());
                            chars.nth(part.len() - 1);

                            variant = separator.clone();

                            continue;
                        }
                        None => {
                            if check.0 {
                                variant = separator.clone();
                            } else if separator.len() > str1.len() {
                                is_end = true;
                            }
                        }
                    }
                }
                None => {
                    is_end = true;
                }
            }
        }

        variant
    }
}

pub fn _gcd_of_strings2(str1: String, str2: String) -> String {
    fn check(str: &str, separator: &str) -> bool {
        let split = str.split(separator).collect::<Vec<&str>>();

        let check_size = split.len() > 1;
        let check_empty = split.iter().all(|part| part.is_empty());

        check_size && check_empty
    }

    if check(str1.as_str(), str2.as_str()) {
        str2
    } else {
        let mut separator = String::new();
        let mut variant = String::new();
        let mut chars = str2.chars();
        let mut is_end = false;
        let mut counter = 0;

        while !is_end {
            counter += 1;
            match chars.next() {
                Some(ch) => {
                    separator.push(ch);
                    if check(str1.as_str(), separator.as_str())
                        && check(str2.as_str(), &separator.as_str())
                    {
                        variant = separator.clone();
                    } else if separator.len() > str1.len() {
                        is_end = true;
                    }
                }
                None => {
                    is_end = true;
                }
            }
        }
        println!("count2 = {}", counter);

        variant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        println!("\n===================== ROUND 1 ===========================\n");
        let word1 = String::from("ABCABC");
        let word2 = String::from("ABC");
        let check = String::from("ABC");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example2() {
        println!("\n===================== ROUND 2 ===========================\n");
        let word1 = String::from("ABABAB");
        let word2 = String::from("ABAB");
        let check = String::from("AB");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example3() {
        println!("\n===================== ROUND 3 ===========================\n");
        let word1 = String::from("LEET");
        let word2 = String::from("CODE");
        let check = String::from("");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example4() {
        println!("\n===================== ROUND 4 ===========================\n");
        let word1 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXX");
        let word2 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX");
        let check = String::from("TAUXX");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example5() {
        println!("\n===================== ROUND 5 ===========================\n");
        let word1 = String::from("NLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGM");
        let word2 = String::from("NLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGMNLZGM");
        let check = String::from("NLZGM");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example6() {
        println!("\n===================== ROUND 6 ===========================\n");
        let word1 = String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        let word2 = String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        let check = String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example7() {
        println!("\n===================== ROUND 7 ===========================\n");
        let word1 = String::from("ABCABCABC");
        let word2 = String::from("ABCAAA");
        let check = String::from("");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example8() {
        println!("\n===================== ROUND 8 ===========================\n");
        let word1 = String::from("EFGABC");
        let word2 = String::from("ABC");
        let check = String::from("");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }

    #[test]
    fn example9() {
        println!("\n===================== ROUND 9 ===========================\n");
        let word1 = String::from("ADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBB");
        let word2 = String::from("ADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBB");
        let check = String::from("ADADCCBBCBDCDDBABCBBADADCCBBCBDCDDBABCBB");

        let result = _gcd_of_strings(word1.clone(), word2.clone());
        let result2 = _gcd_of_strings2(word1, word2);

        assert!(result == check && result2 == check);
    }
}
