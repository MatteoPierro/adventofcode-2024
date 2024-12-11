#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use memoize::memoize;
    use num::Integer;

    #[test]
    fn it_counts_stones_after_blinking_25_times() {
        let input = indoc! {"
            125 17
        "};

        assert_eq!(55312, count_stones_after_blinking(input, 25))
    }

    #[test]
    fn it_solves_puzzles() {
        let input = &read_input_file("input_11");

        assert_eq!(202019, count_stones_after_blinking(input, 25));
        assert_eq!(239321955280205, count_stones_after_blinking(input, 75))
    }

    fn count_stones_after_blinking(input: &str, times: usize) -> usize {
        let lines = read_lines(input);
        let parsed_input = lines[0].split(" ").collect::<Vec<&str>>();

        parsed_input.iter()
            .map(|stone| blink(stone.to_string(), times))
            .sum()
    }

    #[memoize]
    fn blink(stone: String, counter: usize) -> usize {
        if counter == 0 {
            return 1;
        }

        if stone == "0" {
            return blink("1".to_string(), counter - 1);
        }

        let number = stone.parse::<usize>().unwrap();

        if stone.len().is_even() {
            let len = stone.len();
            let exp = (len / 2) as u32;
            let left = number / (10_usize.pow(exp));
            let right = number % (10_usize.pow(exp));
            return blink(left.to_string(), counter - 1) +
                blink(right.to_string(), counter - 1);
        }

        blink((number * 2024).to_string(), counter - 1)
    }
}