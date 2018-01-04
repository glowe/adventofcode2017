extern crate advent;

use advent::get_path_or_exit;
use advent::read_file;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}
use CardinalDirection::*;

fn num_steps(square: u32) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut steps = 0;
    let mut incr = 1;
    let mut direction = East;
    let mut times_matched = 0;
    let mut squares = HashMap::new();
    squares.insert(1, (0, 0));
    for s in 2..(square + 1) {
        match direction {
            East => x += 1,
            North => y += 1,
            West => x -= 1,
            South => y -= 1,
        }
        squares.insert(s, (x, y));

        steps += 1;
        if steps == incr {
            // turn
            steps = 0;
            direction = match direction {
                East => North,
                North => West,
                West => South,
                South => East,
            };
            times_matched += 1;
            if times_matched % 2 == 0 {
                incr += 1;
            }

        }
    }
    x.abs() + y.abs()
}

fn first_value_larger(input: u32) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut steps = 0;
    let mut incr = 1;
    let mut direction = East;
    let mut times_matched = 0;
    let mut squares = HashMap::new();
    let mut values = HashMap::new();
    squares.insert(1, (0, 0));
    values.insert((0, 0), 1);
    let mut value = 0;
    for s in 2.. {
        match direction {
            East => x += 1,
            North => y += 1,
            West => x -= 1,
            South => y -= 1,
        }
        squares.insert(s, (x, y));
        // sum neighbor values
        value = 0;
        // east
        if let Some(v) = values.get(&(x + 1, y)) {
            value += v;
        }
        // north east
        if let Some(v) = values.get(&(x + 1, y + 1)) {
            value += v;
        }
        // north
        if let Some(v) = values.get(&(x, y + 1)) {
            value += v;
        }
        // north west
        if let Some(v) = values.get(&(x - 1, y + 1)) {
            value += v;
        }
        // west
        if let Some(v) = values.get(&(x - 1, y)) {
            value += v;
        }
        // south west
        if let Some(v) = values.get(&(x - 1, y - 1)) {
            value += v;
        }
        // south
        if let Some(v) = values.get(&(x, y - 1)) {
            value += v;
        }
        // south east
        if let Some(v) = values.get(&(x + 1, y - 1)) {
            value += v;
        }

        values.insert((x, y), value);

        if value > input {
            return value;
        }

        steps += 1;
        if steps == incr {
            // turn
            steps = 0;
            direction = match direction {
                East => North,
                North => West,
                West => South,
                South => East,
            };
            times_matched += 1;
            if times_matched % 2 == 0 {
                incr += 1;
            }
        }
    }
    value
}

fn main() {
    let path = get_path_or_exit();
    let input = read_file(&path).unwrap();
    let square = input.trim().parse().unwrap();
    println!("part 1: {}", num_steps(square));
    println!("part 2: {}", first_value_larger(square));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_steps() {
        assert_eq!(0, num_steps(1));
        assert_eq!(3, num_steps(12));
        assert_eq!(2, num_steps(23));
        assert_eq!(31, num_steps(1024));
    }

    #[test]
    fn test_first_value_larger() {
        assert_eq!(2, first_value_larger(1));
        assert_eq!(4, first_value_larger(2));
        assert_eq!(10, first_value_larger(5));
        assert_eq!(11, first_value_larger(10));
        assert_eq!(23, first_value_larger(11));
        assert_eq!(25, first_value_larger(23));
        assert_eq!(26, first_value_larger(25));
        assert_eq!(54, first_value_larger(26));
    }

}
