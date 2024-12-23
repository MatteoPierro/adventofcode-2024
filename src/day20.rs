#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashMap, HashSet, VecDeque};

    #[test]
    fn it_finds_possible_cheat_with_minimal_save() {
        let input = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
        "};

        let map = parse_input(input);


        let distances = map.distances_from_end();
        assert_eq!(84, *distances.get(&map.start).unwrap());
        assert_eq!(5, map.possible_cheat_within_picoseconds(20, 2));
        assert_eq!(1449, map.possible_cheat_within_picoseconds(20, 20));
    }

    #[test]
    fn it_solves_puzzles() {
        let input = &read_input_file("input_20");

        let map = parse_input(input);
        assert_eq!(1367, map.possible_cheat_within_picoseconds(100, 2));
        assert_eq!(1006850, map.possible_cheat_within_picoseconds(100, 20));
    }

    fn parse_input(input: &str) -> Map {
        let lines = read_lines(input);
        let dimensions = (lines.len(), lines.len());
        let mut walls = HashSet::new();
        let mut start = Position(-1, -1);
        let mut end = Position(-1, -1);
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let position = Position(x as isize, y as isize);

                match char {
                    '#' => { walls.insert(position); }
                    'S' => { start = position; }
                    'E' => { end = position; }
                    '.' => {}
                    _ => panic!("invalid")
                }
            }
        }

        Map::new(walls, start, end, dimensions)
    }

    #[derive(Debug, Clone)]
    struct Map {
        walls: HashSet<Position>,
        start: Position,
        end: Position,
        dimensions: (usize, usize),
    }

    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right
    ];

    impl Map {
        fn new(walls: HashSet<Position>, start: Position, end: Position, dimensions: (usize, usize)) -> Self {
            Map { walls, start, end, dimensions }
        }

        fn distances_from_end(&self) -> HashMap<Position, usize> {
            let mut distances = HashMap::new();
            let mut queue = VecDeque::from([(0, self.end.clone())]);
            distances.insert(self.end.clone(), 0);

            while let Some((distance, current)) = queue.pop_front() {
                for n in self.neighbours(&current) {
                    if self.walls.contains(&n) {
                        continue;
                    }

                    if distances.contains_key(&n) {
                        continue;
                    }

                    distances.insert(n.clone(), distance + 1);
                    queue.push_back((distance + 1, n.clone()));
                }
            }

            distances
        }

        fn possible_cheat_within_picoseconds(&self, max_steps: isize, picoseconds: isize) -> usize {
            let mut result = 0;

            let distances = self.distances_from_end();
            for (&ref p1, &d1) in distances.iter() {
                for (&ref p2, &d2) in distances.iter() {
                    let distance_between_points = p1.distance_from(p2);
                    if distance_between_points > picoseconds {
                        continue;
                    }

                    if (d1 as isize - distance_between_points - d2 as isize) < max_steps {
                        continue;
                    }

                    result += 1;
                }
            }

            result
        }

        fn neighbours(&self, position: &Position) -> Vec<Position> {
            DIRECTIONS.iter()
                .map(|d| position.move_one_step(d.clone()))
                .filter(|p| p.is_in_boundaries(self.dimensions))
                .collect()
        }
    }

    #[derive(Debug, Clone)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Position(isize, isize);

    impl Position {
        fn distance_from(&self, other: &Position) -> isize {
            (self.0 - other.0).abs() + (self.1 - other.1).abs()
        }
        fn move_one_step(&self, direction: Direction) -> Position {
            self.move_steps(1, direction)
        }
        fn move_steps(&self, steps: isize, direction: Direction) -> Position {
            match direction {
                Direction::Up => Position(self.0, self.1 - steps),
                Direction::Down => Position(self.0, self.1 + steps),
                Direction::Left => Position(self.0 - steps, self.1),
                Direction::Right => Position(self.0 + steps, self.1),
            }
        }

        fn is_in_boundaries(&self, boundaries: (usize, usize)) -> bool {
            if self.0 < 0 || self.1 < 0 {
                return false;
            }

            if self.0 >= boundaries.0 as isize || self.1 >= boundaries.1 as isize {
                return false;
            }

            true
        }
    }
}