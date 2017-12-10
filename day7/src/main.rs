#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::env;
use std::process;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use regex::Regex;


#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buffer = String::new();
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

lazy_static! {
    static ref RE: regex::Regex = Regex::new(
        r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<children>.+))?$"
    ).unwrap();
}

fn parse(line: &str) -> Program {
    let caps = RE.captures(line).expect("Encountered error with line");
    let name: String = caps.name("name")
        .unwrap()
        .as_str()
        .to_string();
    let weight: u32 = caps.name("weight")
        .unwrap()
        .as_str()
        .parse()
        .expect("Couldn't convert weight to a number.");
    let children: Vec<String> = caps.name("children")
        .map_or_else(Vec::new, |m| {
            m.as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect()

        });
    Program {
        name: name,
        weight: weight,
        children: children,
    }
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
        let program = parse(line);
        if !indegrees.contains_key(&program.name) {
            indegrees.insert(program.name, 0);
        }
        for name in program.children {
            let indegree = indegrees.entry(name).or_insert(1);
            *indegree += 1;
        }
    }

    let root = indegrees.iter()
        .find(|&(_name, &indegree)| indegree == 0)
        .map(|(name, _indegree)| name)
        .unwrap();
    println!("part 1: {}", root);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let program = parse("prog1 (123) -> prog2, prog3, prog4");
        assert_eq!(program.name, "prog1");
        assert_eq!(program.weight, 123);
        assert_eq!(program.children, vec!["prog2", "prog3", "prog4"]);
    }
}
