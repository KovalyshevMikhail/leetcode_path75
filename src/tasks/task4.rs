/// 605. Can Place Flowers
///
/// You have a long flowerbed in which some of the plots are planted, and some are not. However,
/// flowers cannot be planted in adjacent plots.
///
/// Given an integer array flowerbed containing 0's and 1's,
/// where 0 means empty and 1 means not empty, and an integer n,
/// return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule
/// and false otherwise.
///
/// Example 1:
///
/// Input: flowerbed = [1,0,0,0,1], n = 1
/// Output: true
///
pub fn _can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut a = true;
    let mut b = false;
    let mut c = false;

    let mut fill_counter = 1;
    let mut is_end = false;

    let mut need_place = n;

    let mut iter = flowerbed.iter();

    while !is_end {
        if need_place == 0 {
            is_end = true;
            continue;
        }

        match iter.next() {
            Some(is_busy) => {
                if is_busy == &0 {
                    // fill free zone
                    if fill_counter == 0 {
                        a = true;
                        fill_counter += 1;
                    } else if fill_counter == 1 {
                        b = true;
                        fill_counter += 1;
                    } else if fill_counter == 2 {
                        c = true;
                        fill_counter = 0;
                    }

                    // condition for out
                    if a && b && c {
                        need_place -= 1;

                        a = true;
                        b = false;
                        c = false;
                        fill_counter = 1;
                    }
                } else {
                    a = false;
                    b = false;
                    c = false;
                    fill_counter = 0;
                }
            }
            None => {
                is_end = true;
            }
        }
    }

    need_place == 0 || (need_place == 1 && a && b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        println!("\n===================== ROUND 1 ===========================\n");
        let flowerbed = vec![1, 0, 0, 0, 1];
        let extra_flower = 1;
        let check = true;

        let result = _can_place_flowers(flowerbed, extra_flower);

        assert_eq!(result, check);
    }

    #[test]
    fn example2() {
        println!("\n===================== ROUND 2 ===========================\n");
        let flowerbed = vec![1, 0, 0, 0, 1];
        let extra_flower = 2;
        let check = false;

        let result = _can_place_flowers(flowerbed, extra_flower);

        assert_eq!(result, check);
    }

    #[test]
    fn example3() {
        println!("\n===================== ROUND 3 ===========================\n");
        let flowerbed = vec![0, 0, 1, 0, 1];
        let extra_flower = 1;
        let check = true;

        let result = _can_place_flowers(flowerbed, extra_flower);

        assert_eq!(result, check);
    }

    #[test]
    fn example4() {
        println!("\n===================== ROUND 4 ===========================\n");
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let extra_flower = 2;
        let check = false;

        let result = _can_place_flowers(flowerbed, extra_flower);

        assert_eq!(result, check);
    }

    #[test]
    fn example5() {
        println!("\n===================== ROUND 5 ===========================\n");
        let flowerbed = vec![1, 0, 0, 0, 0, 0, 1];
        let extra_flower = 2;
        let check = true;

        let result = _can_place_flowers(flowerbed, extra_flower);

        assert_eq!(result, check);
    }

    #[test]
    fn example6() {
        println!("\n===================== ROUND 6 ===========================\n");
        let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
        let extra_flower = 2;
        let check = true;

        let result = _can_place_flowers(flowerbed, extra_flower);

        assert_eq!(result, check);
    }
}
