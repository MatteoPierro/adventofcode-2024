#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use num::Integer;

    #[test]
    fn it_counts_stones_after_blinking_25_times() {
        let input = indoc! {"
            125 17
        "};

        let result = count_stones_after_blinking_25_times(input);
        assert_eq!(55312, result)
    }

    #[test]
    fn it_solves_fist_puzzle() {
        let input = &read_input_file("input_11");

        let result = count_stones_after_blinking_25_times(input);
        assert_eq!(202019, result)
    }

    fn count_stones_after_blinking_25_times(input: &str) -> usize {
        let lines = read_lines(input);
        let parsed_input = lines[0].split(" ").collect::<Vec<&str>>();

        let result: usize = parsed_input.iter()
            .map(|stone| blink(stone, 25))
            .sum();
        result
    }

    fn blink(stone: &str, counter: usize) -> usize {
        if counter == 0 {
            return 1;
        }

        if stone == "0" {
            return blink("1", counter - 1);
        }

        let number = stone.parse::<usize>().unwrap();

        if stone.len().is_even() {
            let len = stone.len();
            let exp = (len / 2) as u32;
            let left = number / (10_usize.pow(exp));
            let right = number % (10_usize.pow(exp));
            return blink(&left.to_string(), counter - 1) + blink(&right.to_string(), counter - 1);
        }

        blink(&(number * 2024).to_string(), counter - 1)
    }
}