#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_parse_the_line() {
        assert_eq!(vec![7, 6, 4, 2, 1], parse_line("7 6 4 2 1"))
    }

    #[test]
    fn it_parse_the_input() {
        let input = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"};
        assert_eq!(
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
            parse_input(&input)
        )
    }

    #[test]
    fn it_counts_safe_report() {
        let input = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9"};
        assert_eq!(2, count_safe_report(&input))
    }

    #[test]
    fn it_solves_the_first_puzzle() {
        let input = read_input_file("input_02");
        assert_eq!(624, count_safe_report(&input))
    }

    #[test]
    fn it_checks_that_report_is_safe() {
        assert_eq!(true, is_report_safe(&vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_report_safe(&vec![1, 2, 7, 8, 9]));
        assert_eq!(false, is_report_safe(&vec![1, 3, 2, 4, 5]))
    }

    fn count_safe_report(input: &str) -> usize {
        parse_input(input).iter()
            .filter(|&r| is_report_safe(r) )
            .count()
    }

    fn is_report_safe(line: &Vec<usize>) -> bool {
        let diffs: Vec<_> = line.windows(2)
            .map(|pair| pair[1] as isize - pair[0] as isize)
            .collect();
        (diffs.iter().all(|d| d.is_positive()) || diffs.iter().all(|d| d.is_negative()))
            && diffs.iter().all(|d| d.abs() > 0 && d.abs() < 4)
    }

    fn parse_input(input: &str) -> Vec<Vec<usize>> {
        read_lines(input).iter()
            .map(|line| parse_line(line))
            .collect()
    }


    fn parse_line(line: &str) -> Vec<usize> {
        line.split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    }
}