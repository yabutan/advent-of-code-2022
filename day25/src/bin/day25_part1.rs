use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let sum: i64 = r.lines().flatten().map(|l| convert_to_normal(&l)).sum();
    println!("sum: {}", sum);

    let sum_snafu = convert_to_snafu(sum);
    println!("sum_snafu: {}", sum_snafu);
}

fn convert_to_normal(n_snafu: &str) -> i64 {
    const K: i64 = 5;

    let mut n = 0;
    for (i, r) in n_snafu.chars().rev().enumerate() {
        let place = K.pow(i as u32);

        match r {
            '0' | '1' | '2' => {
                let r: i64 = r.to_digit(10).unwrap() as i64;
                n += r * place;
            }
            '=' => {
                n += -2 * place;
            }
            '-' => {
                n += -place;
            }
            _ => unreachable!("Invalid character"),
        }
    }

    n
}

fn convert_to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }

    const K: i64 = 5;
    let mut v: Vec<char> = Vec::new();
    loop {
        if n == 0 {
            break;
        }

        let r = n % K;

        match r {
            0 | 1 | 2 => {
                v.push(r.to_string().chars().next().unwrap());
                n /= K;
            }
            3 => {
                v.push('=');
                n /= K;
                n += 1 // -2 として扱うため、繰り上げ
            }
            4 => {
                v.push('-');
                n /= K;
                n += 1 // -1 として扱うため、繰り上げ
            }
            _ => panic!("Unexpected remainder: {}", r),
        }
    }
    v.reverse();
    v.iter().map(|c| format!("{}", c)).join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn test_sample1() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let sum: i64 = r.lines().flatten().map(|l| convert_to_normal(&l)).sum();
        assert_eq!(sum, 4890);

        let sum_snafu = convert_to_snafu(sum);
        assert_eq!(sum_snafu, "2=-1=0");
    }

    #[test]
    fn test_convert_to_normal() {
        assert_eq!(convert_to_normal("0"), 0);
        assert_eq!(convert_to_normal("1"), 1);
        assert_eq!(convert_to_normal("2"), 2);
        assert_eq!(convert_to_normal("1="), 3);
        assert_eq!(convert_to_normal("1-"), 4);
        assert_eq!(convert_to_normal("10"), 5);
        assert_eq!(convert_to_normal("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_convert_to_snafu() {
        assert_eq!(convert_to_snafu(0), "0");
        assert_eq!(convert_to_snafu(1), "1");
        assert_eq!(convert_to_snafu(2), "2");
        assert_eq!(convert_to_snafu(3), "1=");
        assert_eq!(convert_to_snafu(4), "1-");
        assert_eq!(convert_to_snafu(5), "10");
        assert_eq!(convert_to_snafu(314159265), "1121-1110-1=0");
    }
}
