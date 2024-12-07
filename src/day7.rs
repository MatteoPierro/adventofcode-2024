#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::input_reader::{read_input_file, read_lines};


    type Combiner = fn(usize, usize) -> usize;

    fn sum(a: usize, b: usize) -> usize {
        a + b
    }

    fn mul(a: usize, b: usize) -> usize {
        a * b
    }

    fn join(a: usize, b: usize) -> usize {
        format!("{}{}", a.to_string(), b.to_string()).parse::<usize>().unwrap()
    }

    struct Equation(usize, Vec<usize>);

    impl Equation {
        fn is_valid(&self, combiners: &Vec<Combiner>) -> bool {
            let current = self.1[0];
            self.is_valid_rec(current, 1, combiners)
        }

        fn is_valid_rec(&self, current: usize, next_index: usize, combiners: &Vec<Combiner>) -> bool {
            if self.1.len() == next_index && current == self.0 {
                return true;
            }

            if self.1.len() == next_index {
                return false;
            }

            combiners.iter()
                .any(|c|
                    self.is_valid_rec(c(current, self.1[next_index]), next_index + 1, combiners)
                )
        }
    }

    #[test]
    fn it_validates_an_equation() {
        let equation = Equation(3267, vec![81, 40, 27]);
        assert!(equation.is_valid(&vec![sum, mul]))
    }

    #[test]
    fn it_parses_the_input() {
        let input = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20"};

        assert_eq!(3749, total_calibration_result(input, &vec![sum, mul]))
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_07");

        assert_eq!(4122618559853, total_calibration_result(input, &vec![sum, mul]))
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_07");

        assert_eq!(227615740238334, total_calibration_result(input, &vec![sum, mul, join]))
    }

    fn total_calibration_result(input: &str, combiners: &Vec<Combiner>) -> usize {
        read_lines(input)
            .iter()
            .map(|l| {
                let s = l.split(": ").collect::<Vec<_>>();
                let result = s[0].parse::<usize>().unwrap();
                let factors = s[1].split(" ")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect();
                Equation(result, factors)
            })
            .filter(|e| e.is_valid(combiners))
            .map(|s| s.0)
            .sum::<usize>()
    }
}