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

fn main() {
    let path = get_path_or_exit();
    let input = read_file(&path).unwrap();
    let lengths: Vec<usize> = input
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let mut vec: Vec<i32> = (0..256).collect();
    knot_hash(&mut vec, &lengths);
    println!("part 1: {}", vec[0] * vec[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

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
