#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use indoc::indoc;
    use crate::input_reader::{read_input_file, read_lines};

    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Debug)]
    struct Map {
        guard: (usize, usize),
        obstacles: HashSet<(usize, usize)>,
        dimensions: (usize, usize),
    }

    impl Map {
        fn from(input: &str) -> Self {
            let map = read_lines(input)
                .iter()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let dimensions = (map[0].len(), map.len());
            let mut guard = None;
            let mut obstacles = HashSet::new();

            for (y, line) in map.iter().enumerate() {
                for (x, &char) in line.iter().enumerate() {
                    match char {
                        '^' => guard = Some((x, y)),
                        '#' => { obstacles.insert((x, y)); }
                        _ => ()
                    }
                }
            }

            Self { guard: guard.unwrap(), obstacles, dimensions }
        }

        fn walk(&self) -> usize {
            let mut current_position = (self.guard.0 as isize, self.guard.1 as isize);
            let mut direction = Direction::North;
            let mut visited = HashSet::new();

            while self.is_in_boundaries(&mut current_position) {
                visited.insert(current_position);

                let new_position = match direction {
                    Direction::North => (current_position.0, current_position.1 - 1),
                    Direction::East => (current_position.0 + 1, current_position.1),
                    Direction::South => (current_position.0, current_position.1 + 1),
                    Direction::West => (current_position.0 - 1, current_position.1)
                };

                if self.obstacles.contains(&(new_position.0 as usize, new_position.1 as usize)) {
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North
                    };
                    continue;
                }

                current_position = new_position
            }

            visited.len()
        }

        fn is_in_boundaries(&self, position: &(isize, isize)) -> bool {
            position.0 >= 0 && position.0 < (self.dimensions.0 as isize)
                && position.1 >= 0 && position.1 < (self.dimensions.1 as isize)
        }
    }

    #[test]
    fn it_calculates_the_steps() {
        let input = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#..."};

        let map = Map::from(input);

        assert_eq!(41, map.walk())
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_06");

        let map = Map::from(input);

        assert_eq!(5444, map.walk())
    }
}