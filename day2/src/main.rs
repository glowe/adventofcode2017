use std::io::{self, Read};

fn max_delta(row: &[u32]) -> u32 {
    let min: &u32 = row.iter().min().unwrap();
    let max: &u32 = row.iter().max().unwrap();
    max - min
}

fn find_quotient(rows: &mut Vec<u32>) -> u32 {
    rows.sort_by(|a, b| b.cmp(a));
    for i in 0..rows.len() {
        for j in (i+1)..rows.len() {
            if rows[i] % rows[j] == 0 {
                return rows[i] / rows[j];
            }
        }
    }
    unreachable!();
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut sum_delta: u32 = 0;
    let mut sum_quotient: u32 = 0;
    for line in buffer.trim().lines() {
        let mut row: Vec<u32> = line.split_whitespace().map(|number| number.parse().unwrap()).collect();
        sum_delta += max_delta(&row);
        sum_quotient += find_quotient(&mut row);
    }
    println!("part 1: {}", sum_delta);
    println!("part 2: {}", sum_quotient);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_delta() {
        assert_eq!(8, max_delta(&vec![5, 1, 9, 5]));
        assert_eq!(4, max_delta(&vec![7, 5, 3]));
        assert_eq!(6, max_delta(&vec![2, 4, 6, 8]));
    }

    #[test]
    fn test_find_quotient() {
        assert_eq!(4, find_quotient(&mut vec![5, 9, 2, 8]));
        assert_eq!(3, find_quotient(&mut vec![9, 4, 7, 3]));
        assert_eq!(2, find_quotient(&mut vec![3, 8, 6, 4]));
    }
}
