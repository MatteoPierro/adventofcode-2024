#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::input_reader::{read_input_file, read_lines};

    struct Equation(usize, Vec<usize>);

    impl Equation {
        fn is_valid(&self) -> bool {
            let current = self.1[0];
            self.is_valid_rec(current, 1)
        }

        fn is_valid_rec(&self, current: usize, next_index: usize) -> bool {
            if self.1.len() == next_index && current == self.0 {
                return true;
            }

            if self.1.len() == next_index {
                return false;
            }

            self.is_valid_rec(current + self.1[next_index], next_index + 1) ||
                self.is_valid_rec(current * self.1[next_index], next_index + 1)
        }
    }

    #[test]
    fn it_validates_an_equation() {
        let equation = Equation(3267, vec![81, 40, 27]);
        assert!(equation.is_valid())
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

        assert_eq!(3749, total_calibration_result(input))
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_07");

        assert_eq!(4122618559853, total_calibration_result(input))
    }

    fn total_calibration_result(input: &str) -> usize {
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
            .filter(|e| e.is_valid())
            .map(|s| s.0)
            .sum::<usize>()
    }
}