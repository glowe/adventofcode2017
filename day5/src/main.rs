use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, Read, Result};

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer: String = String::new();
    buf_reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn to_i32s(contents: &str) -> Vec<i32> {
    contents.split_whitespace()
        .map(|digits| digits.parse().unwrap())
        .collect()
}

fn count_steps_to_exit<F>(jump_offsets: &mut Vec<i32>, get_incr: F) -> u32
    where F: Fn(i32) -> i32
{
    let mut i: i32 = 0;
    let mut steps: u32 = 0;
    while 0 <= i && i < jump_offsets.len() as i32 {
        let jump: i32 = jump_offsets[i as usize];
        jump_offsets[i as usize] += get_incr(jump);
        steps += 1;
        i += jump
    }
    steps
}

fn plus_one(_: i32) -> i32 {
    1
}

fn plus_or_minus_one(j: i32) -> i32 {
    if j >= 3 {
        -1
    } else {
        1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <input>", args[0]);
        process::exit(1);
    }
    let contents = read_file(&args[1]).expect("Couldn't read input file");
    let mut jump_offsets = to_i32s(&contents);
    println!(
        "part 1: {}",
        count_steps_to_exit(&mut jump_offsets, plus_one)
    );
    let mut more_jump_offsets = to_i32s(&contents);
    println!(
        "part 2: {}",
        count_steps_to_exit(&mut more_jump_offsets, plus_or_minus_one)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_steps_to_exit() {
        assert_eq!(
            count_steps_to_exit(&mut vec![0, 3, 0, 1, -3], plus_one),
            5
        );
        assert_eq!(
            count_steps_to_exit(&mut vec![0, 3, 0, 1, -3], plus_or_minus_one),
            10
        );
    }
}
