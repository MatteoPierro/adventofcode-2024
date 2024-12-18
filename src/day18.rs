#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn it_finds_minimum_steps_to_take() {
        let input = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        "};

        assert_eq!(Some(22), find_minimum_steps_to_take(input, 12, 6));
    }

    #[test]
    fn it_finds_first_coordinate_preventing_reaching_end() {
        let input = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
        "};

        assert_eq!("6,1", first_coordinate_preventing_reaching_end(input, 12, 6))
    }

    #[test]
    fn it_solves_both_puzzles() {
        let input = &read_input_file("input_18");

        assert_eq!(Some(312), find_minimum_steps_to_take(input, 1024, 70));
        assert_eq!("28,26", first_coordinate_preventing_reaching_end(input, 1024, 70))
    }

    fn first_coordinate_preventing_reaching_end(input: &str, start: usize, range: isize) -> String {
        let corrupted_bytes = read_lines(input);
        let mut start = start;
        let mut end = corrupted_bytes.len();

        while (start + end) / 2 != start {
            if find_minimum_steps_to_take(input, (start + end) / 2, range).is_some() {
                start = (start + end) / 2;
            } else {
                end = (start + end) / 2;
            }
        }

        let first_coordinate_preventing_reaching_end = corrupted_bytes[end - 1].clone();
        first_coordinate_preventing_reaching_end
    }

    fn find_minimum_steps_to_take(input: &str, steps_to_take: usize, range: isize) -> Option<usize> {
        let mut corrupted_bytes = HashSet::new();
        for line in read_lines(input).iter().take(steps_to_take) {
            let raw_digits = line.split(",").collect::<Vec<_>>();
            let x = raw_digits[0].parse::<isize>().unwrap();
            let y = raw_digits[1].parse::<isize>().unwrap();
            corrupted_bytes.insert((x, y));
        }

        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), 0));

        let mut visited = HashSet::new();

        while let Some((position, steps)) = queue.pop_front() {
            if position == (range, range) {
                return Some(steps);
            }

            if visited.contains(&position) {
                continue;
            }

            visited.insert(position.clone());

            for n in neighbours(position, range) {
                if corrupted_bytes.contains(&n) {
                    continue;
                }

                queue.push_back((n, steps + 1))
            }
        }

        None
    }

    fn neighbours((x, y): (isize, isize), range: isize) -> Vec<(isize, isize)> {
        let mut result = vec![];

        if x + 1 <= range {
            result.push((x + 1, y))
        }

        if x - 1 >= 0 {
            result.push((x - 1, y))
        }

        if y + 1 <= range {
            result.push((x, y + 1))
        }

        if y - 1 >= 0 {
            result.push((x, y - 1))
        }

        result
    }
}