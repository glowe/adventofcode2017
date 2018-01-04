extern crate advent;

use advent::get_path_or_exit;
use advent::read_file;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(input: &str) -> Vec<Vec<&str>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.trim())
                .collect()
        })
        .collect()
}

fn is_passphrase_valid(passphrase: &[&str]) -> bool {
    let mut words: HashSet<_> = HashSet::new();
    for word in passphrase {
        if words.contains(&word) {
            return false;
        }
        words.insert(word);
    }
    true
}

fn normalize(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    String::from_iter(chars.iter())
}

fn main() {
    let path = get_path_or_exit();
    let input = read_file(&path).expect("Had trouble reading input file.");
    let passphrases = parse(&input);
    let num_without_dupes = passphrases.iter()
        .filter(|passphrase| is_passphrase_valid(passphrase))
        .count();
    println!("part 1: {}", num_without_dupes);

    let normed: Vec<Vec<String>> = passphrases.iter()
        .map(|passphrase| {
            passphrase.iter()
                .map(|word| normalize(word))
                .collect()
        })
        .collect();
    let normed_refs: Vec<Vec<&str>> = normed.iter()
        .map(|passphrase| {
            passphrase.iter()
                .map(|word| word.as_ref())
                .collect()
        })
        .collect();

    let num_without_anagrams = normed_refs.iter()
        .filter(|passphrase| is_passphrase_valid(passphrase))
        .count();
    println!("part 2: {}", num_without_anagrams);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_aa_bb_cc_dd_ee() {
        assert_eq!(vec![vec!["aa", "bb", "cc", "dd", "ee"], vec!["hello", "world"]],
                   parse("aa bb cc dd ee\nhello world"));
    }

    #[test]
    fn test_aa_bb_cc_dd_ee() {
        assert_eq!(true,
                   is_passphrase_valid(&vec!["aa", "bb", "cc", "dd", "ee"]));
    }

    #[test]
    fn test_normalize() {
        assert_eq!("abcd", normalize("bcad"));
    }
}
