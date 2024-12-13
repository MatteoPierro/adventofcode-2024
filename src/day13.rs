#[cfg(test)]
mod tests {
    use std::cmp;
    use indoc::indoc;
    use memoize::memoize;
    use regex::Regex;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_calculates_tokens() {
        let button_a = (94, 34);
        let button_b = (22, 67);
        let prize = (8400, 5400);
        assert_eq!(Some(280), calculate_tokens(button_a, button_b, prize));

        let button_a = (26, 66);
        let button_b = (67, 21);
        let prize = (12748, 12176);
        assert_eq!(None, calculate_tokens(button_a, button_b, prize));

        let button_a = (17, 86);
        let button_b = (84, 37);
        let prize = (7870, 6450);
        assert_eq!(Some(200), calculate_tokens(button_a, button_b, prize));
    }

    #[test]
    fn it_calculates_total_tokens() {
        let input = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
        "};

        assert_eq!(480, calculate_total_tokens(input));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_13");

        assert_eq!(38839, calculate_total_tokens(input)); // 9 seconds
    }

    fn calculate_total_tokens(input: &str) -> usize {
        let lines = read_lines(input);

        let mut total_tokens = 0;
        let mut index = 0;
        while index < lines.len() {
            if lines[index] == "" {
                index += 1;
                continue;
            }


            let button_a = parse_button(&lines[index]);
            // println!("button_a {:?}", button_a);
            let button_b = parse_button(&lines[index + 1]);
            // println!("button_b {:?}", button_b);
            let prize = parse_prize(&lines[index + 2]);
            // println!("prize {:?}", prize);

            if let Some(token) = calculate_tokens(button_a, button_b, prize) {
                total_tokens += token
            }
            index += 3;
        }
        total_tokens
    }

    fn parse_button(raw_button: &String) -> (isize, isize) {
        let button_pattern = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        parse_tuple(raw_button, button_pattern)
    }

    fn parse_prize(raw_button: &String) -> (isize, isize) {
        let button_pattern = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        parse_tuple(raw_button, button_pattern)
    }

    fn parse_tuple(line: &String, pattern: Regex) -> (isize, isize) {
        pattern
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [first, second])| {
                (first.parse::<isize>().unwrap(), second.parse::<isize>().unwrap())
            }).next().unwrap()
    }

    #[memoize]
    fn calculate_tokens(button_a: (isize, isize), button_b: (isize, isize), prize: (isize, isize)) -> Option<usize> {
        if prize.0 == 0 && prize.1 == 0 {
            return Some(0);
        }

        if prize.0 < 0 || prize.1 < 0 {
            return None;
        }

        let a = calculate_tokens(button_a, button_b, (prize.0 - button_a.0, prize.1 - button_a.1));
        let b = calculate_tokens(button_a, button_b, (prize.0 - button_b.0, prize.1 - button_b.1));

        match (a, b) {
            (Some(a), Some(b)) => Some(cmp::min(3 + a, 1 + b)),
            (Some(a), _) => Some(3 + a),
            (_, Some(b)) => Some(1 + b),
            (_, _) => None
        }
    }
}