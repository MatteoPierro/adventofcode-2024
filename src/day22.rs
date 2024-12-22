#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashMap, HashSet};

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

    #[test]
    fn it_calculates_most_bananas() {
        let input = indoc! {"
        1
        2
        3
        2024
        "};

        assert_eq!(23, calculate_most_bananas(input));
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_22");

        assert_eq!(2272, calculate_most_bananas(input)); // slow ~47 seconds
    }

    fn calculate_most_bananas(input: &str) -> usize {
        let times_to_evolve = 2000;

        let all_changes = read_lines(input).iter()
            .map(|raw_secret_code| {
                let secret_code = raw_secret_code.parse::<usize>().unwrap();
                calculate_changes(times_to_evolve, secret_code)
            })
            .collect::<Vec<_>>();

        let all_combinations = all_changes.iter()
            .fold(HashSet::new(), |mut acc, current| {
                acc.extend(current.keys());
                acc
            });

        all_combinations
            .iter()
            .map(|&combination| {
                all_changes.iter()
                    .map(|changes|
                        changes.get(combination).unwrap_or(&0)
                    ).sum()
            }).max()
            .unwrap()
    }

    fn calculate_changes(times_to_evolve: i32, secret_code: usize) -> HashMap<Vec<isize>, usize> {
        let (_, bananas, deltas) = (0..times_to_evolve)
            .fold(
                (secret_code, vec![], vec![]),
                |(prev, mut bananas, mut deltas), _| {
                    let next = evolve(prev);
                    bananas.push(next % 10);
                    deltas.push((next % 10) as isize - (prev % 10) as isize);
                    (next, bananas, deltas)
                });

        let mut result = HashMap::new();

        for (index, w) in deltas.windows(4).enumerate() {
            let key = w.to_vec();

            if result.contains_key(&key) {
                continue;
            }

            result.insert(key, bananas[index + 3]);
        }

        result
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