use day23::State;

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let mut state = State::read_from(r);
    while state.do_round() {}

    println!("answer:{:?}", state.get_round());
}
