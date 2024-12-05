#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use indoc::indoc;
    use itertools::Itertools;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_sums_middle_page_number_for_correct_updates() {
        let input = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"};

        let (preconditions, updates) = parse_input(input);

        assert_eq!(
            143,
            sum_middle_page_number_for_correct_updates(&preconditions, updates)
        )
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_05");

        let (preconditions, updates) = parse_input(input);

        assert_eq!(
            5639,
            sum_middle_page_number_for_correct_updates(&preconditions, updates)
        )
    }

    fn sum_middle_page_number_for_correct_updates(
        preconditions: &HashMap<usize, Vec<usize>>,
        updates: Vec<Vec<usize>>,
    ) -> usize {
        updates.iter()
            .filter(|u| is_update_valid(&preconditions, u))
            .map(|u| u[u.len() / 2])
            .sum()
    }

    fn is_update_valid(preconditions: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
        update.iter().enumerate().all(|(index, value)| {
            if let Some(p) = preconditions.get(value) {
                (index + 1..update.len()).all(|j| !p.contains(&update[j]))
            } else {
                true
            }
        })
    }

    fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
        let (raw_rules, raw_updates): (Vec<_>, Vec<_>) = read_lines(input).into_iter()
            .partition(|l| l.contains("|"));

        let preconditions = raw_rules.iter()
            .fold(HashMap::new(), |mut acc, rr| {
                let (left, right): (usize, usize) = rr.split("|")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                acc.entry(right)
                    .and_modify(|pre: &mut Vec<_>| pre.push(left))
                    .or_insert(vec![left]);
                acc
            });

        let updates = raw_updates.iter()
            .skip(1)
            .map(|ru| {
                ru.split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect()
            }).collect();

        (preconditions, updates)
    }
}