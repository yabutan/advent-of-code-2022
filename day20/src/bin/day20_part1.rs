use std::fmt::{Debug, Display};
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let list = read_input(r);
    let mut mixer = Mixer::new(list);
    for i in 0..mixer.size {
        mixer.mix(i);
    }

    let numbers = mixer.find_three_numbers();
    let sum: i64 = numbers.iter().sum();

    println!("result: {:?} => {}", numbers, sum);
}

fn read_input<T>(r: impl BufRead) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    r.lines()
        .map(|line| {
            let line = line.unwrap();
            line.parse::<T>().unwrap()
        })
        .collect()
}

#[derive(Debug)]
struct Mixer {
    list: Vec<i64>,
    indices: Vec<usize>,
    size: usize,
}

impl Mixer {
    fn new(list: Vec<i64>) -> Self {
        let size = list.len();
        let indices: Vec<usize> = (0..size).collect();
        Self {
            list,
            indices,
            size,
        }
    }

    fn display_list(&self) -> Vec<i64> {
        self.indices.iter().map(|i| self.list[*i]).collect()
    }

    fn mix(&mut self, i: usize) {
        let v = self.list[i];

        let from = self.indices.iter().position(|&x| x == i).unwrap();
        self.indices.remove(from);

        let to = calc_index(from, self.indices.len(), v);
        if to == 0 {
            self.indices.push(i);
        } else {
            self.indices.insert(to, i);
        }
    }

    fn find_three_numbers(&self) -> Vec<i64> {
        let index_of_zero = self.list.iter().position(|&x| x == 0).unwrap();
        let index_of_zero = self
            .indices
            .iter()
            .position(|&x| x == index_of_zero)
            .unwrap();

        let a = self.list[self.indices[calc_index(index_of_zero, self.size, 1000)]];
        let b = self.list[self.indices[calc_index(index_of_zero, self.size, 2000)]];
        let c = self.list[self.indices[calc_index(index_of_zero, self.size, 3000)]];

        vec![a, b, c]
    }
}

fn calc_index(index: usize, len: usize, d: i64) -> usize {
    let new_index = index as i64 + d;
    new_index.rem_euclid(len as i64) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let list: Vec<i64> = read_input(r);
        let mut mixer = Mixer::new(list);

        assert_eq!(mixer.display_list(), vec![1, 2, -3, 3, -2, 0, 4]);

        mixer.mix(0);
        assert_eq!(mixer.display_list(), vec![2, 1, -3, 3, -2, 0, 4]);

        mixer.mix(1);
        assert_eq!(mixer.display_list(), vec![1, -3, 2, 3, -2, 0, 4]);

        mixer.mix(2);
        assert_eq!(mixer.display_list(), vec![1, 2, 3, -2, -3, 0, 4]);

        mixer.mix(3);
        assert_eq!(mixer.display_list(), vec![1, 2, -2, -3, 0, 3, 4]);

        mixer.mix(4);
        assert_eq!(mixer.display_list(), vec![1, 2, -3, 0, 3, 4, -2]);

        mixer.mix(5);
        assert_eq!(mixer.display_list(), vec![1, 2, -3, 0, 3, 4, -2]);

        mixer.mix(6);
        assert_eq!(mixer.display_list(), vec![1, 2, -3, 4, 0, 3, -2]);

        let numbers = mixer.find_three_numbers();
        assert_eq!(numbers, vec![4, -3, 2]);

        let sum: i64 = numbers.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_read_from_input() {
        let list: Vec<i32> = read_input(include_str!("../../data/sample.txt").as_bytes());
        assert_eq!(list.len(), 7);
        assert_eq!(list[0], 1);
        assert_eq!(list[1], 2);
        assert_eq!(list[2], -3);
        assert_eq!(list[6], 4);
    }

    #[test]
    fn test_calc_index() {
        assert_eq!(calc_index(0, 3, 0), 0);
        assert_eq!(calc_index(0, 3, 3), 0);
        assert_eq!(calc_index(2, 3, 2), 1);
        assert_eq!(calc_index(0, 3, -1), 2);
        assert_eq!(calc_index(0, 3, -3), 0);
    }
}
