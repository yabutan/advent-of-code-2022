use day21::{read_expr, Calculator};

fn main() {
    let r = include_str!("../../data/input.txt").as_bytes();

    let expr_list = read_expr(r);
    let calculator = Calculator::new(expr_list);

    println!("result: {:?}", calculator.calc("root"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample2() {
        let r = include_str!("../../data/sample.txt").as_bytes();

        let expr_list = read_expr(r);
        let calculator = Calculator::new(expr_list);

        assert_eq!(calculator.calc("root"), Some(152));
    }
}
