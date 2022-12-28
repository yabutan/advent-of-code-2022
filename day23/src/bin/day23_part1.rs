use day23::State;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let mut state = State::read_from(r);
    for _ in 0..10 {
        state.do_round();
    }

    println!("answer:{:?}", state.count_of_spaces());
}
