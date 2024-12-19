#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;

    #[test]
    fn it_counts_possible_designs() {
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

        assert_eq!(6, count_possible_designs(input));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_19");

        assert_eq!(242, count_possible_designs(input));
    }

    fn count_possible_designs(input: &str) -> usize {
        let lines = read_lines(input);

        let patterns = lines[0].clone()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut possible_designs = 0;
        for index in 2..lines.len() {
            let design = lines[index].clone();
            if valid_patter(design.clone(), &patterns) {
                possible_designs += 1;
            }
        }
        possible_designs
    }

    fn valid_patter(design: String, patterns: &Vec<String>) -> bool {
        if design.is_empty() {
            return true;
        }

        if patterns.contains(&design) {
            return true;
        }

        for i in 0..design.len() {
            let part = design[0..=i].to_string();
            if !patterns.contains(&part) {
                continue;
            }

            if valid_patter(design[i + 1..design.len()].to_string(), patterns) {
                return true;
            }
        }
        false
    }
}