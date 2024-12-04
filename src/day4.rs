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
            .flat_map(|s| navigate(&parsed_input, s))
            .filter(|w| w == "XMAS")
            .count()
    }

    fn navigate(parsed_input: &Vec<Vec<char>>, start: (isize, isize)) -> Vec<String> {
        [
            [(0, 0), (0, 1), (0, 2), (0, 3)],
            [(0, 0), (0, -1), (0, -2), (0, -3)],
            [(0, 0), (1, 0), (2, 0), (3, 0)],
            [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
            [(0, 0), (1, 1), (2, 2), (3, 3)],
            [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
            [(0, 0), (1, -1), (2, -2), (3, -3)],
            [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
        ].iter()
            .map(|d| navigate_to_direction(&parsed_input, start, *d))
            .collect()
    }

    fn navigate_to_direction(map: &Vec<Vec<char>>, (x, y): (isize, isize), direction: [(isize, isize); 4]) -> String {
        direction.iter().filter_map(|(dx, dy)| {
            map.get((y + *dy) as usize)?.get((x + *dx) as usize)
        }).collect()
    }
}