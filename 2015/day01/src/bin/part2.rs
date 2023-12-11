fn main() {
    let input = include_str!("input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let result = input.lines().map(|line| {
        let mut partly = 0;
        let mut idxs = Vec::new();
        for (i,c) in line.chars().enumerate() {
            if c == '(' {
                partly += 1;
            } else {
                partly -= 1;
            }
            if partly < 0 {
                idxs.push(i.to_owned());
            }
        }
        let res = idxs.first().unwrap().to_owned();
        res

    }).collect::<Vec<_>>().first().unwrap().to_owned();

    let r = result + 1;
    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
        "()())"
            );
        assert_eq!(result, "5".to_string());
    }
}
