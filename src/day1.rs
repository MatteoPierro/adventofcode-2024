#[cfg(test)]
mod tests {
    use indoc::indoc;
    use itertools::Itertools;
    use crate::input_reader::*;

    #[test]
    fn it_parses_the_input() {
        let input = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "};
        let (c1, c2) = parse_input(&input);
        assert_eq!(vec![3, 4, 2, 1, 3, 3], c1);
        assert_eq!(vec![4, 3, 5, 3, 9, 3], c2);
    }

    #[test]
    fn it_parses_a_line() {
        let line = "3   4";
        assert_eq!((3, 4), parse_line(line))
    }

    #[test]
    fn it_calculates_the_total_distance() {
        let input = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3"};
        assert_eq!(11, calculate_total_distance(input))
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_01");
        assert_eq!(3246517, calculate_total_distance(input))
    }

    #[test]
    fn it_calculates_similarity_score() {
        let input = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3"};
        assert_eq!(31, calculate_similarity_score(input))
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_01");
        assert_eq!(29379307, calculate_similarity_score(input))
    }

    fn calculate_similarity_score(input: &str) -> usize {
        let (c1, c2) = parse_input(input);
        let frequencies = c2.iter().counts();
        c1.iter()
            .map(|v1| v1 * frequencies.get(v1).unwrap_or(&0))
            .sum()
    }

    fn calculate_total_distance(input: &str) -> usize {
        let (mut c1, mut c2) = parse_input(input);
        c1.sort();
        c2.sort();
        c1.iter()
            .zip(c2)
            .map(|(v1, v2)| v1.abs_diff(v2))
            .sum()
    }

    fn parse_line(line: &str) -> (usize, usize) {
        line.split("   ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    }

    fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
        read_lines(input).iter()
            .map(|line| parse_line(line))
            .fold((vec![], vec![]), |(mut c1, mut c2), (v1, v2)| {
                c1.push(v1);
                c2.push(v2);
                (c1, c2)
            })
    }
}