#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::HashMap;

    #[test]
    fn it_solve_for_test_input() {
        let input = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
        "};

        assert_eq!((6, 16), solve(input));
    }

    #[test]
    fn it_solves_both_puzzle() {
        let input = &read_input_file("input_19");

        assert_eq!((242, 595975512785325), solve(input));
    }

    fn solve(input: &str) -> (usize, usize) {
        let lines = read_lines(input);

        let patterns = lines[0].clone()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut memo: HashMap<String, usize> = HashMap::new();
        let mut possible_designs = 0;
        let mut all_different_ways = 0;
        for index in 2..lines.len() {
            let design = lines[index].clone();
            let design_combinations = possible_combinations_per_pattern(design.clone(), &patterns, &mut memo);
            if design_combinations > 0 {
                possible_designs += 1;
            }
            all_different_ways += design_combinations;
        }
        (possible_designs, all_different_ways)
    }

    fn possible_combinations_per_pattern(design: String, patterns: &Vec<String>, memo: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if let Some(&result) = memo.get(&design) {
            return result;
        }

        let mut possible_combinations = 0;
        for i in 0..design.len() {
            let part = design[0..i + 1].to_string();
            if !patterns.contains(&part) {
                continue;
            }

            possible_combinations += possible_combinations_per_pattern(design[i + 1..design.len()].to_string(), patterns, memo);
        }
        memo.insert(design.clone(), possible_combinations);
        possible_combinations
    }
}