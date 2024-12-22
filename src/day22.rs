#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;

    #[test]
    fn it_evolve_a_secret_number() {
        assert_eq!(15887950, evolve(123))
    }

    #[test]
    fn it_calculates_sum_of_the_2000th_generated_secret_number() {
        let input = indoc! {"
        1
        10
        100
        2024
        "};

        assert_eq!(37327623, calculate_sum_of_the_2000th_generated_secret_number(input))
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_22");

        assert_eq!(20401393616, calculate_sum_of_the_2000th_generated_secret_number(input))
    }

    fn calculate_sum_of_the_2000th_generated_secret_number(input: &str) -> usize {
        let mut secret_numbers: Vec<usize> = read_lines(input).iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        for _ in 0..2000 {
            for i in 0..secret_numbers.len() {
                secret_numbers[i] = evolve(secret_numbers[i])
            }
        }

        let sum: usize = secret_numbers.iter().sum();
        sum
    }

    fn evolve(secret_number: usize) -> usize {
        let mut secret_number = ((secret_number << 6) ^ secret_number) % 16777216;
        secret_number = ((secret_number >> 5) ^ secret_number) % 16777216;
        secret_number = ((secret_number << 11) ^ secret_number) % 16777216;
        secret_number
    }
}