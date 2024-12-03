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

    fn mul_sum_line(line: &str) -> usize {
        Regex::new(r"mul\((\d+),(\d+)\)").unwrap()
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [first, second])| first.parse::<usize>().unwrap() * second.parse::<usize>().unwrap())
            .sum()
    }
}