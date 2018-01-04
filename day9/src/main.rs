extern crate advent;

use advent::get_path_or_exit;
use advent::read_file;

fn score(input: &str) -> (u32, u32) {
    let mut stack: Vec<char> = Vec::new();
    let mut score: u32 = 0;
    let mut garbage: bool = false;
    let mut ignore_next: bool = false;
    let mut num_garbage: u32 = 0;
    for ch in input.chars() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        match ch {
            '!' => ignore_next = true,
            '<' => if garbage {
                num_garbage += 1;
            } else {
                garbage = true;
            },
            '>' => if garbage {
                garbage = false;
            },
            '{' => if !garbage {
                stack.push(ch);
            } else {
                num_garbage += 1;
            },
            '}' => {
                if !garbage {
                    if stack.len() > 0 && stack[stack.len() - 1] == '{' {
                        score += stack.len() as u32;
                        stack.pop();
                    }
                } else {
                    num_garbage += 1;
                }
            }
            _ => if garbage {
                num_garbage += 1;
            },
        }
    }
    (score, num_garbage)
}

fn main() {
    let path = get_path_or_exit();
    let input = read_file(&path).unwrap();
    println!("part 1: {}", score(&input).0);
    println!("part 2: {}", score(&input).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score("{}").0, 1);
        assert_eq!(score("{{{}}}").0, 6);
        assert_eq!(score("{{{},{},{{}}}}").0, 16);
        assert_eq!(score("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn test_garbage() {
        assert_eq!(score("<>").1, 0);
        assert_eq!(score("<random characters>").1, 17);
        assert_eq!(score("<<<<>").1, 3);
        assert_eq!(score("<{!>}>").1, 2);
        assert_eq!(score("<!!>").1, 0);
        assert_eq!(score("<!!!>>").1, 0);
        assert_eq!(score("<{o\"i!a,<{i<a>").1, 10);
    }
}
