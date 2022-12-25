use day21::{read_expr, Calculator};

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let expr_list = read_expr(r);
    let calculator = Calculator::new_with_unknown_humn(expr_list);

    println!("result: {:?}", calculator.resolve_humn());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let expr_list = read_expr(r);
        let calculator = Calculator::new_with_unknown_humn(expr_list);

        assert_eq!(calculator.resolve_humn(), 301);
    }
}
