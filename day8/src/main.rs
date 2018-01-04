#![feature(slice_patterns)]
#![feature(match_default_bindings)]
use std::collections::HashMap;
use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::process;

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn execute<'a>(reg: &'a str, inc: &str, delta: i32, registers: &mut HashMap<&'a str, i32>) {
    let delta = match inc {
        "dec" => -delta,
        "inc" => delta,
        _ => unreachable!(),
    };
    *registers.entry(reg).or_insert(0) += delta;
}

fn run<'a>(input: &'a str, mut registers: &mut HashMap<&'a str, i32>) -> Option<u32> {
    let mut highest: Option<u32> = None;
    for line in input.lines() {
        // register instruction(inc/dec) amount if register operator value
        let terms = line.split_whitespace().collect::<Vec<&str>>();
        let reg: &str = terms[0];
        let inc: &str = terms[1];
        let delta: i32 = terms[2].parse().unwrap();
        let reg_operand: &str = terms[4];
        let op: &str = terms[5];
        let val_operand: i32 = terms[6].parse().unwrap();
        let reg_value: i32 = *registers.get(reg_operand).or(Some(&0)).unwrap();
        let comparison_true = match op {
            "<" => reg_value < val_operand,
            "<=" => reg_value <= val_operand,
            ">" => reg_value > val_operand,
            ">=" => reg_value >= val_operand,
            "!=" => reg_value != val_operand,
            "==" => reg_value == val_operand,
            _ => unreachable!(),
        };
        if comparison_true {
            execute(reg, inc, delta, &mut registers);
            if let Some(x) = registers.values().max() {
                highest = Some(cmp::max(highest.unwrap_or(*x as u32), *x as u32));
            }
        }
    }
    highest
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <input>", args[0]);
        process::exit(1);
    }
    let path = &args[1];
    let input = read_file(path).unwrap();
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let highest_value = run(&input, &mut registers);
    println!("part 1: {}", registers.values().max().unwrap());
    println!("part 2: {}", highest_value.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        let mut registers: HashMap<&str, i32> = HashMap::new();
        run(&input, &mut registers);
        assert_eq!(*registers.values().max().unwrap(), 1);
    }
}
