#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_calculates_the_sum_of_multiplications() {
        let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, mul_sum_line(line))
    }

    #[test]
    fn it_solve_first_puzzle() {
        let total: usize = read_lines(&read_input_file("input_03"))
            .iter().map(|l| mul_sum_line(l))
            .sum();
        assert_eq!(187194524, total)
    }

    #[test]
    fn it_sums_only_enabled() {
        let line = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, sum_only_enabled(line))
    }

    #[test]
    fn it_solve_second_puzzle() {
        let line = read_lines(&read_input_file("input_03")).join("");
        assert_eq!(127092535, sum_only_enabled(&line))
    }

    fn sum_only_enabled(line: &str) -> usize {
        Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap()
            .captures_iter(line)
            .map(|c| c.extract())
            .fold((0, true), |(sum, take), (_, [instruction])| {
                if instruction == "do()" {
                    return (sum, true)
                }

                if instruction == "don't()" || take == false {
                    return (sum, false)
                }

                (sum + mul_sum_line(instruction), true)
            }).0
    }

    fn mul_sum_line(line: &str) -> usize {
        Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [first, second])| {
                first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap()
            })
            .sum()
    }
}