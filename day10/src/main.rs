#![feature(inclusive_range, inclusive_range_syntax)]
extern crate advent;

use advent::get_path_or_exit;
use advent::read_file;

fn creverse<T: std::fmt::Debug>(vec: &mut [T], start: usize, length: usize) {
    if length == 0 {
        return;
    }
    let mut i = start;
    let mut l = length - 1;
    loop {
        let j = (i + l) % vec.len();
        vec.swap(i, j);
        i = (i + 1) % vec.len();
        if l <= 2 {
            break;
        } else {
            l -= 2;
        }
    }
}

fn knot_hash<T: std::fmt::Debug>(mut vec: &mut [T], lengths: &[usize]) {
    let mut pos = 0;
    for (skip, length) in lengths.iter().enumerate() {
        creverse(&mut vec, pos, *length);
        pos = (pos + *length + skip) % vec.len();
    }
}

fn sparse_hash<T: std::fmt::Debug>(mut vec: &mut [T], lengths: &[u8]) {
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for length in lengths.iter() {
            let length = *length as usize;
            creverse(&mut vec, pos, length);
            pos = (pos + length + skip) % vec.len();
            skip += 1;
        }
    }
}

fn csv_to_lengths(csv: &str) -> Vec<usize> {
    csv.trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn ascii_codes(input: &str) -> Vec<u8> {
    let mut codes: Vec<u8> = input.trim().bytes().collect();
    codes.extend_from_slice(&[17, 31, 73, 47, 23]);
    codes
}

fn compact(vec: Vec<u8>) -> Vec<u8> {
    vec.chunks(16)
        .map(|v| {
            v[0] ^ v[1] ^ v[2] ^ v[3] ^ v[4] ^ v[5] ^ v[6] ^ v[7] ^ v[8] ^ v[9] ^ v[10] ^ v[11]
                ^ v[12] ^ v[13] ^ v[14] ^ v[15]
        })
        .collect()
}

fn main() {
    let path = get_path_or_exit();
    let input = read_file(&path).unwrap();
    let lengths = csv_to_lengths(&input);
    let mut vec: Vec<u8> = (0..=255).collect();
    knot_hash(&mut vec, &lengths);
    println!("part 1: {}", vec[0] as u32 * vec[1] as u32);

    let ascii_code_lengths = ascii_codes(&input.trim());
    let mut vec: Vec<u8> = (0..=255).collect();
    sparse_hash(&mut vec, &ascii_code_lengths);
    let dense_hash = compact(vec);
    let hex: Vec<String> = dense_hash.iter().map(|n| format!("{:02x}", n)).collect();
    println!("part 2: {}", hex.join(""));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_codes() {
        assert_eq!(
            ascii_codes("1,2,3"),
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
        );
        assert_eq!(
            ascii_codes("63,144,180,149"),
            vec![54, 51, 44, 49, 52, 52, 44, 49, 56, 48, 44, 49, 52, 57]
        )
    }

    #[test]
    fn test_creverse_one() {
        let mut vec = vec![0, 1, 2, 3, 4];
        creverse(&mut vec, 1, 1);
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_creverse_two() {
        let mut vec = vec![0, 1, 2, 3, 4];
        creverse(&mut vec, 1, 2);
        assert_eq!(vec, vec![0, 2, 1, 3, 4]);
    }

    #[test]
    fn test_creverse_three() {
        let mut vec = vec![0, 1, 2, 3, 4];
        creverse(&mut vec, 1, 3);
        assert_eq!(vec, vec![0, 3, 2, 1, 4]);
    }

    #[test]
    fn test_creverse_five() {
        let mut vec = vec![0, 1, 2, 3, 4];
        creverse(&mut vec, 1, 5);
        assert_eq!(vec, vec![1, 0, 4, 3, 2]);
    }

    #[test]
    fn test_creverse_start_gt_length() {
        let mut vec = vec![0, 1, 2, 3, 4];
        creverse(&mut vec, 4, 1);
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
        creverse(&mut vec, 4, 2);
        assert_eq!(vec, vec![4, 1, 2, 3, 0]);
    }

    #[test]
    fn test_knot_hash() {
        let mut list = vec![0, 1, 2, 3, 4];
        let lengths = vec![3, 4, 1, 5];
        knot_hash(&mut list, &lengths);
        assert_eq!(list, vec![3, 4, 2, 1, 0]);
    }
}
