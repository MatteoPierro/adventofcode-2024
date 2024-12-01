#[cfg(test)]
mod tests {
    use std::ptr::read;
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