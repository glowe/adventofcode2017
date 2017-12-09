extern crate regex;

use std::collections::HashMap;
use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, Read, Result};

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buffer = String::new();
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse(line: &str) -> (&str, Vec<&str>) {
    let mut split: Vec<_> = line.trim().split(" -> ").collect();
    let program_and_weight = split.remove(0);
    let program = program_and_weight.split_whitespace().next().unwrap();
    let programs: Vec<&str> = if let Some(csv) = split.pop() {
        csv.split(", ").collect()
    } else {
        Vec::new()
    };
    (program, programs)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {}", args[0]);
        process::exit(1);
    }
    let path = &args[1];
    let input = read_file(path).unwrap();
    let mut indegrees: HashMap<String, u32> = HashMap::new();

    for line in input.lines() {
        let (program, programs) = parse(line);
        if !indegrees.contains_key(program) {
            indegrees.insert(program.to_owned(), 0);
        }
        for program in programs {
            let indegree = indegrees.entry(program.to_owned()).or_insert(1);
            *indegree += 1;
        }
    }

    let root = indegrees.iter()
        .find(|&(_program, &indegree)| indegree == 0)
        .map(|(program, _indegree)| program)
        .unwrap();
    println!("part 1: {}", root);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("prog1 (123) -> prog2, prog3, prog4"),
                   ("prog1", vec!["prog2", "prog3", "prog4"]));
    }
}
