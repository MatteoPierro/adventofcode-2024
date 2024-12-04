#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_count_xmas() {
        let input = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"};

        assert_eq!(18, count_xmas(input))
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_04");

        assert_eq!(2718, count_xmas(input))
    }

    #[test]
    fn it_count_x_mas() {
        let input = indoc! {"
        .M.S......
        ..A..MSMS.
        .M.S.MAA..
        ..A.ASMSM.
        .M.S.M....
        ..........
        S.S.S.S.S.
        .A.A.A.A..
        M.M.M.M.M.
        .........."};

        assert_eq!(9, count_x_mas(input))
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_04");

        assert_eq!(2046, count_x_mas(input))
    }

    fn count_x_mas(input: &str) -> usize {
        let parsed_input: Vec<_> = read_lines(input).iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();

        parsed_input
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(|(x, &char)| {
                    if char == 'A' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            })
            .filter(|&s| filter_x_mas(&parsed_input, s))
            .count()
    }

    fn filter_x_mas(parsed_input: &Vec<Vec<char>>, start: (isize, isize)) -> bool {
        let words = [
            vec![(-1, -1), (0, 0), (1, 1)],
            vec![(-1, 1), (0, 0), (1, -1)],
        ].iter()
            .map(|d| navigate_to_direction(&parsed_input, start, d))
            .collect::<Vec<_>>();

        if words.len() != 2 {
            return false
        }

        words.iter().all(|w| w == "MAS" || w == "SAM")
    }

    fn count_xmas(input: &str) -> usize {
        let parsed_input: Vec<_> = read_lines(input).iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();

        parsed_input
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(|(x, &char)| {
                    if char == 'X' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            })
            .flat_map(|s| navigate_xmas(&parsed_input, s))
            .filter(|w| w == "XMAS")
            .count()
    }

    fn navigate_xmas(parsed_input: &Vec<Vec<char>>, start: (isize, isize)) -> Vec<String> {
        [
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
            vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
            vec![(0, 0), (1, -1), (2, -2), (3, -3)],
            vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        ].iter()
            .map(|d| navigate_to_direction(&parsed_input, start, d))
            .collect()
    }

    fn navigate_to_direction(map: &Vec<Vec<char>>, (x, y): (isize, isize), direction: &Vec<(isize, isize)>) -> String {
        direction.iter().filter_map(|(dx, dy)| {
            map.get((y + *dy) as usize)?.get((x + *dx) as usize)
        }).collect()
    }
}