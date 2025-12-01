use std::fs;

static DAY: &str = "00";

fn main() {
    println!("day{}", DAY);
    let (result1, result2) = (0, 0);

    let input = fs::read_to_string(format!("../input/day{}.txt", DAY)).unwrap();
    let _split = input.split_whitespace();

    // part 1
    println!(">> part 1: {}", result1);

    // part 2
    println!(">> part 2: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#;

    #[test]
    fn test_test_input() {
        let _vec_str = TEST_INPUT.split_whitespace();

        todo!()
    }
}
