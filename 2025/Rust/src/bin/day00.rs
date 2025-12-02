use std::fs;

static DAY: u8 = 0;

fn main() {
    println!("day{DAY}");

    let input = fs::read_to_string(format!("../input/day{DAY}.txt")).unwrap();
    let _split = input.split_whitespace();

    // part 1
    let result1 = 0;
    println!(">> part 1: {}", result1);

    // part 2
    let result2 = 0;
    println!(">> part 2: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
        "#;
    const TEST_SOLUTION: usize = 0;
    const TEST_SOLUTION_PART2: usize = 0;

    #[test]
    fn test_test_input() {
        let _vec_str = TEST_INPUT.split_whitespace();

        todo!()
    }
}
