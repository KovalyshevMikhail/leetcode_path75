/// 345. Reverse Vowels of a String
///
/// Given a string s, reverse only all the vowels in the string and return it.
///
/// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
///
/// Example 1:
///
/// Input: s = "hello"
/// Output: "holle"
///

pub fn _reverse_vowels(s: String) -> String {
    if s.len() == 1 {
        s
    } else {
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        let text_size = s.len();
        let mut iter_forward: std::iter::Enumerate<std::str::Chars<'_>> = s.chars().enumerate();
        let mut iter_backward: std::iter::Enumerate<std::iter::Rev<std::str::Chars<'_>>> =
            s.chars().rev().enumerate();

        let mut is_end = false;

        let mut stop_forward = false;
        let mut stop_backward = false;

        let mut part_left = String::new();
        let mut part_right = String::new();

        let (mut forward_index, mut forward_ch) = (0, ' ');
        let (mut backward_index, mut backward_ch) = (0, ' ');

        while !is_end {
            if !stop_forward {
                match iter_forward.next() {
                    Some(next) => {
                        forward_index = next.0;
                        forward_ch = next.1;
                    }
                    None => {
                        is_end = true;
                        continue;
                    }
                }
            }
            if !stop_backward {
                match iter_backward.next() {
                    Some(next) => {
                        backward_index = next.0;
                        backward_ch = next.1;
                    }
                    None => {
                        is_end = true;
                        continue;
                    }
                }
            }

            if forward_index > text_size - backward_index - 1 {
                is_end = true;
                continue;
            }
            if forward_index == text_size - backward_index - 1 {
                if !stop_forward {
                    part_left.push(forward_ch);
                } else if !stop_backward {
                    part_right.push(backward_ch);
                }
                is_end = true;
                stop_forward = false;
                stop_backward = false;
                continue;
            }

            // println!(
            //     "[{}]   '{}'[{}] <===> '{}'[{}]    [{}]",
            //     part_left,
            //     forward_ch,
            //     forward_index,
            //     backward_ch,
            //     text_size - backward_index - 1,
            //     part_right
            // );

            if vowels.contains(&forward_ch) {
                stop_forward = true;
            }
            if vowels.contains(&backward_ch) {
                stop_backward = true;
            }

            if !stop_forward {
                part_left.push(forward_ch);
            }

            if !stop_backward {
                part_right.push(backward_ch);
            }

            if stop_forward && stop_backward {
                part_left.push(backward_ch);
                part_right.push(forward_ch);

                stop_forward = false;
                stop_backward = false;
            }
        }

        if stop_forward {
            part_left.push(forward_ch);
        }

        if stop_backward {
            part_right.push(backward_ch);
        }

        // println!("{} === {}", part_left, part_right);
        part_left.push_str(&part_right.chars().rev().collect::<String>().as_str());
        part_left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        println!("\n===================== ROUND 1 ===========================\n");
        let text = String::from("hello");
        let check = String::from("holle");

        let result = _reverse_vowels(text);

        assert_eq!(result, check);
    }

    #[test]
    fn example2() {
        println!("\n===================== ROUND 2 ===========================\n");
        let text = String::from("leetcode");
        let check = String::from("leotcede");

        let result = _reverse_vowels(text);

        assert_eq!(result, check);
    }

    #[test]
    fn example3() {
        println!("\n===================== ROUND 3 ===========================\n");
        let text = String::from(" ");
        let check = String::from(" ");

        let result = _reverse_vowels(text);

        assert_eq!(result, check);
    }

    #[test]
    fn example4() {
        println!("\n===================== ROUND 4 ===========================\n");
        let text = String::from("a.");
        let check = String::from("a.");

        let result = _reverse_vowels(text);

        assert_eq!(result, check);
    }

    #[test]
    fn example5() {
        println!("\n===================== ROUND 5 ===========================\n");
        let text = String::from("!!!");
        let check = String::from("!!!");

        let result = _reverse_vowels(text);

        assert_eq!(result, check);
    }

    #[test]
    fn example6() {
        println!("\n===================== ROUND 6 ===========================\n");
        let text = String::from(".a");
        let check = String::from(".a");

        let result = _reverse_vowels(text);

        assert_eq!(result, check);
    }
}
