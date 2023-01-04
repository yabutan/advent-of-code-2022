use std::io::BufRead;

use crate::operation::read_operations;
pub use crate::operation::Operation;
use crate::stack::read_stacks;

mod operation;
mod stack;

pub fn simulate(
    mut r: impl BufRead,
    mover: fn(stacks: &mut [Vec<char>], op: &Operation),
) -> String {
    let mut stacks = read_stacks(&mut r);
    let operations = read_operations(r);

    for op in operations {
        mover(&mut stacks, &op);
    }

    // read top of stacks
    stacks.iter().filter_map(|s| s.last()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        let mut r = include_str!("../data/sample.txt").as_bytes();
        let stacks = read_stacks(&mut r);
        let operations = read_operations(r);

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks[2], vec!['P']);

        assert_eq!(operations.len(), 4);
        assert_eq!(
            operations[0],
            Operation {
                times: 1,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            operations[3],
            Operation {
                times: 1,
                from: 1,
                to: 2
            }
        );
    }
}
