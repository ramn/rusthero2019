fn main() {
    let args: Vec<String> = std::env::args().skip(1).take(2).collect();
    println!("{}", solve(&args[0], args[1].parse().unwrap()));
}

fn solve(input: &str, rounds: usize) -> String {
    let mut xs = parse(input);
    for _ in 0..rounds {
        assert!(xs.len() > 0);
        let mut buf = vec![];
        let mut current = xs[0];
        let mut count = 1;
        for &d in xs.iter().skip(1) {
            if d == current {
                count += 1;
            } else {
                buf.push(count);
                buf.push(current);
                count = 1;
                current = d;
            }
        }
        buf.push(count);
        buf.push(current);
        std::mem::swap(&mut xs, &mut buf);
    }
    xs.iter().map(|d| format!("{}", d)).collect()
}

fn parse(input: &str) -> Vec<usize> {
    input.chars().map(|c| c.to_string().parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve("1", 1), "11");
        assert_eq!(solve("2", 1), "12");
        assert_eq!(solve("11", 1), "21");
        assert_eq!(solve("31", 1), "1311");
        assert_eq!(solve("3211", 1), "131221");
        assert_eq!(solve("111223", 1), "312213");
        assert_eq!(solve("111223", 2), "1311221113");
        // assert_eq!(solve("22164224441", 40), "1");
    }
}
