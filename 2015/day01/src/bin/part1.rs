fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = input.lines().map(|line| {
        let mut partly = 0;
        for c in line.chars() {
            if c == '(' {
                partly += 1;
            } else {
                partly -= 1;
            }
        }
        partly
    }).sum::<i32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
        "(()(()("
            );
        assert_eq!(result, "3".to_string());
    }
}
