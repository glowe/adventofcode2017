use std::io;

fn sum_match_offset(captcha: &str, offset: usize) -> u32 {
    let digits: Vec<_> = captcha.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let len = digits.len();
    (0..len).fold(0, |sum, i| {
        if digits[i] == digits[(i + offset) % len] {
            sum + digits[i]
        } else {
            sum
        }
    })
}

fn sum_match_next(captcha: &str) -> u32 {
    sum_match_offset(&captcha, 1)
}

fn sum_match_halfway_around(captcha: &str) -> u32 {
    sum_match_offset(&captcha, captcha.len() / 2)
}

fn main() {
    let mut captcha = String::new();
    io::stdin().read_line(&mut captcha).unwrap();
    // pop the newline.
    captcha.pop();
    println!("part one: {}", sum_match_next(&captcha));
    println!("part two: {}", sum_match_halfway_around(&captcha));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_match_next() {
        assert_eq!(3, sum_match_next("1122"));
        assert_eq!(4, sum_match_next("1111"));
        assert_eq!(0, sum_match_next("1234"));
        assert_eq!(9, sum_match_next("91212129"));
    }

    #[test]
    fn test_sum_match_halfway_around() {
        assert_eq!(6, sum_match_halfway_around("1212"));
        assert_eq!(0, sum_match_halfway_around("1221"));
        assert_eq!(4, sum_match_halfway_around("123425"));
        assert_eq!(12, sum_match_halfway_around("123123"));
        assert_eq!(4, sum_match_halfway_around("12131415"));
    }
}
