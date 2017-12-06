use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // Strip the newline.
    input.pop();
    let captcha: Vec<u32> = parse(&input);
    println!("part one: {}", sum_match_next(&captcha));
    println!("part two: {}", sum_match_halfway_around(&captcha));
}

fn parse(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn sum_match_next(captcha: &Vec<u32>) -> u32 {
    sum_match_offset(captcha, 1)
}

fn sum_match_halfway_around(captcha: &Vec<u32>) -> u32 {
    sum_match_offset(captcha, captcha.len() / 2)
}

fn sum_match_offset(captcha: &Vec<u32>, offset: usize) -> u32 {
    let n = captcha.len();
    (0..n)
        .map(|i| if captcha[i] == captcha[(i + offset) % n] {
            captcha[i]
        } else {
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("1111"), vec![1, 1, 1, 1])
    }

    #[test]
    fn test_sum_match_next() {
        assert_eq!(3, sum_match_next(&vec![1, 1, 2, 2]));
        assert_eq!(4, sum_match_next(&vec![1, 1, 1, 1]));
        assert_eq!(0, sum_match_next(&vec![1, 2, 3, 4]));
        assert_eq!(9, sum_match_next(&vec![9, 1, 2, 1, 2, 1, 2, 9]));
    }

    #[test]
    fn test_sum_match_halfway_around() {
        assert_eq!(6, sum_match_halfway_around(&vec![1, 2, 1, 2]));
        assert_eq!(0, sum_match_halfway_around(&vec![1, 2, 2, 1]));
        assert_eq!(4, sum_match_halfway_around(&vec![1, 2, 3, 4, 2, 5]));
        assert_eq!(12, sum_match_halfway_around(&vec![1, 2, 3, 1, 2, 3]));
        assert_eq!(4, sum_match_halfway_around(&vec![1, 2, 1, 3, 1, 4, 1, 5]));
    }
}
