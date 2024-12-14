#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use regex::Regex;

    #[test]
    fn it_calculates_tokens() {
        let button_a = (94, 34);
        let button_b = (22, 67);
        let prize = (8400, 5400);

        assert_eq!(Some(280), calculate_tokens_with_math(button_a, button_b, prize));

        let button_a = (26, 66);
        let button_b = (67, 21);
        let prize = (12748, 12176);
        assert_eq!(None, calculate_tokens_with_math(button_a, button_b, prize));

        let button_a = (17, 86);
        let button_b = (84, 37);
        let prize = (7870, 6450);
        assert_eq!(Some(200), calculate_tokens_with_math(button_a, button_b, prize));
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

        assert_eq!(480, calculate_total_tokens(input, false));
    }

    #[test]
    fn it_solves_puzzles() {
        let input = &read_input_file("input_13");

        assert_eq!(38839, calculate_total_tokens(input, false));
        assert_eq!(75200131617108, calculate_total_tokens(input, true)); // too high
    }

    fn calculate_tokens_with_math(button_a: (isize, isize), button_b: (isize, isize), prize: (isize, isize)) -> Option<usize> {
        let den = button_a.1 * button_b.0 - button_a.0 * button_b.1;
        if den == 0 {
            return None;
        }

        let b = (prize.0 * button_a.1 - button_a.0 * prize.1) / den;
        let a = (prize.1 - button_b.1 * b) / button_a.1;
        if a * button_a.0 + b * button_b.0 != prize.0 {
            return None;
        }

        if a * button_a.1 + b * button_b.1 != prize.1 {
            return None;
        }

        Some((3 * a + b) as usize)
    }

    fn calculate_total_tokens(input: &str, extend: bool) -> usize {
        let lines = read_lines(input);

        let mut total_tokens = 0;
        let mut index = 0;
        while index < lines.len() {
            if lines[index] == "" {
                index += 1;
                continue;
            }


            let button_a = parse_button(&lines[index]);
            let button_b = parse_button(&lines[index + 1]);
            let prize = parse_prize(&lines[index + 2], extend);

            if let Some(token) = calculate_tokens_with_math(button_a, button_b, prize) {
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

    fn parse_prize(raw_button: &String, extend: bool) -> (isize, isize) {
        let button_pattern = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        let (x, y) = parse_tuple(raw_button, button_pattern);
        if !extend {
            return (x, y);
        }

        let ext_x = 10000000000000 + x;
        let ext_y = 10000000000000 + y;
        (ext_x, ext_y)
    }

    fn parse_tuple(line: &String, pattern: Regex) -> (isize, isize) {
        pattern
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [first, second])| {
                (first.parse::<isize>().unwrap(), second.parse::<isize>().unwrap())
            }).next().unwrap()
    }
}