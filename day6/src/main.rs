use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::process;
use std::collections::HashMap;

fn position_of_max(vec: &[u32]) -> usize {
    let max = vec.iter().max().unwrap();
    assert!(*max > 0);
    vec.iter().position(|x| *x == *max).unwrap()
}


fn count_redistribution_cycles(banks: &mut Vec<u32>, num_repeats: u32) -> u32 {
    let mut configs = HashMap::new();
    let mut cycles = 0;
    loop {
        let n = *configs.get(&banks.to_owned()).or(Some(&0)).unwrap() + 1;
        configs.insert(banks.to_owned(), n);
        if n == num_repeats {
            return cycles;
        }

        let mut i = position_of_max(banks);
        let mut blocks = banks[i];
        banks[i] = 0;
        cycles += 1;

        // distribute blocks
        while blocks > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            blocks -= 1;
        }

    }
}

fn parse(input: &str) -> Vec<u32> {
    input.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buffer = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <input>", args[0]);
        process::exit(1);
    }
    let path = &args[1];
    let input = read_file(path).unwrap();
    let banks = parse(&input);
    let cycles = count_redistribution_cycles(&mut banks.clone(), 2);
    println!("part 1: {}", cycles);
    let cycles2 = count_redistribution_cycles(&mut banks.clone(), 3);
    println!("part 2: {}", cycles2 - cycles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_of_max() {
        assert_eq!(position_of_max(&vec![1, 3, 5, 4, 2, 0, 5]), 2);
    }

    #[test]
    fn test_count_redistribution_cycles() {
        assert_eq!(count_redistribution_cycles(&mut vec![0, 2, 7, 0], 2), 5);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("14	0	15	12	11	11	3	5	1	6	8	4	9	1	8	4
"),
                   vec![14, 0, 15, 12, 11, 11, 3, 5, 1, 6, 8, 4, 9, 1, 8, 4]);
    }
}
