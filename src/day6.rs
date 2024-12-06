#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::HashSet;

    #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        fn step(&self, position: &(isize, isize)) -> (isize, isize) {
            match self {
                Direction::North => (position.0, position.1 - 1),
                Direction::East => (position.0 + 1, position.1),
                Direction::South => (position.0, position.1 + 1),
                Direction::West => (position.0 - 1, position.1)
            }
        }

        fn rotate(&self) -> Self {
            match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            }
        }
    }

    #[derive(Debug)]
    struct Map {
        guard: (isize, isize),
        obstacles: HashSet<(isize, isize)>,
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
                        '^' => guard = Some((x as isize, y as isize)),
                        '#' => { obstacles.insert((x as isize, y as isize)); }
                        _ => ()
                    }
                }
            }

            Self { guard: guard.unwrap(), obstacles, dimensions }
        }

        fn walk(&self) -> HashSet<(isize, isize)> {
            let mut current_position = self.guard;
            let mut direction = Direction::North;
            let mut visited = HashSet::new();

            while self.is_in_boundaries(&mut current_position) {
                visited.insert(current_position);

                let new_position = direction.step(&current_position);

                if self.obstacles.contains(&new_position) {
                    direction = direction.rotate();
                    continue;
                }

                current_position = new_position
            }

            visited
        }

        fn is_in_boundaries(&self, position: &(isize, isize)) -> bool {
            position.0 >= 0 && position.0 < (self.dimensions.0 as isize)
                && position.1 >= 0 && position.1 < (self.dimensions.1 as isize)
        }

        fn count_obfuscations(&self) -> usize {
            self.walk()
                .iter()
                .filter(|&&w| self.is_looping(w))
                .count()
        }

        fn is_looping(&self, new_wall: (isize, isize)) -> bool {
            if new_wall == self.guard {
                return false
            }

            let mut loops: HashSet<((isize, isize), Direction)> = HashSet::new();
            let mut current_position = self.guard;
            let mut direction = Direction::North;

            while self.is_in_boundaries(&current_position) {
                if loops.contains(&(current_position, direction)) {
                    return true
                }

                loops.insert((current_position, direction));

                let new_position = direction.step(&current_position);

                if self.obstacles.contains(&new_position) || new_position == new_wall {
                    direction = direction.rotate();
                    continue;
                }

                current_position = new_position
            }

            false
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

        assert_eq!(6, map.count_obfuscations());
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_06");

        let map = Map::from(input);

        assert_eq!(5444, map.walk().len())
    }

    #[test]
    fn it_counts_obfuscations() {
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

        assert_eq!(6, map.count_obfuscations());
    }

    // Really really slow ~ 24s
    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_06");

        let map = Map::from(input);

        assert_eq!(1946, map.count_obfuscations());
    }
}