use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let list = read_input(r);
    let mut mixer = Mixer::new_with_decryption_key(list, 811589153);

    // ten times
    for _ in 0..10 {
        for i in 0..mixer.size {
            mixer.mix(i);
        }
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
    decryption_key: i64,
}

impl Mixer {
    fn new(list: Vec<i64>) -> Self {
        let size = list.len();
        let indices: Vec<usize> = (0..size).collect();
        Self {
            list,
            indices,
            size,
            decryption_key: 1,
        }
    }

    fn new_with_decryption_key(list: Vec<i64>, decryption_key: i64) -> Self {
        let size = list.len();
        let indices: Vec<usize> = (0..size).collect();
        Self {
            list,
            indices,
            size,
            decryption_key,
        }
    }

    fn display_list(&self) -> Vec<i64> {
        let index_of_zero = self.list.iter().position(|&x| x == 0).unwrap();
        let index_of_zero = self
            .indices
            .iter()
            .position(|&x| x == index_of_zero)
            .unwrap();

        (0..self.size)
            .into_iter()
            .map(|i| {
                self.list[self.indices[calc_index(index_of_zero, self.size, i as i64)]]
                    * self.decryption_key
            })
            .collect()
    }

    fn mix(&mut self, i: usize) {
        let v = self.list[i];

        let from = self.indices.iter().position(|&x| x == i).unwrap();
        self.indices.remove(from);

        let to = calc_index(from, self.indices.len(), v * self.decryption_key);
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

        let a = self.list[self.indices[calc_index(index_of_zero, self.size, 1000)]]
            * self.decryption_key;

        let b = self.list[self.indices[calc_index(index_of_zero, self.size, 2000)]]
            * self.decryption_key;

        let c = self.list[self.indices[calc_index(index_of_zero, self.size, 3000)]]
            * self.decryption_key;

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
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let list: Vec<i64> = read_input(r);
        let mut mixer = Mixer::new_with_decryption_key(list, 811589153);

        // Initial arrangement
        assert_eq!(
            mixer.display_list(),
            vec![
                0,
                3246356612,
                811589153,
                1623178306,
                -2434767459,
                2434767459,
                -1623178306,
            ]
        );

        // After 1 round of mixing
        for i in 0..mixer.size {
            mixer.mix(i);
        }
        assert_eq!(
            mixer.display_list(),
            vec![
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153
            ]
        );

        // After 10 round of mixing
        for _ in 1..10 {
            for i in 0..mixer.size {
                mixer.mix(i);
            }
        }
        assert_eq!(
            mixer.display_list(),
            vec![
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153
            ]
        );

        let numbers = mixer.find_three_numbers();
        assert_eq!(numbers, vec![811589153, 2434767459, -1623178306]);

        let sum: i64 = numbers.iter().sum();
        assert_eq!(sum, 1623178306);
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
