#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn it_works() {
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


        assert_eq!(Some(84), map.shortest_path(HashSet::new(), map.start, usize::MAX));
        assert_eq!(5, map.possible_cheat_with_minimal_save(84, 20));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_20");

        let map = parse_input(input);


        assert_eq!(Some(9464), map.shortest_path(HashSet::new(), map.start, usize::MAX));
        assert_eq!(1367, map.possible_cheat_with_minimal_save(9464, 100)); // slow ~ 1,5 minutes
    }

    fn parse_input(input: &str) -> Map {
        let lines = read_lines(input);
        let dimensions = (lines.len(), lines.len());
        let mut walls = HashSet::new();
        let mut start = (-1, -1);
        let mut end = (-1, -1);
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let position = (x as isize, y as isize);

                if char == '#' {
                    walls.insert(position);
                }

                if char == 'S' {
                    start = position;
                }

                if char == 'E' {
                    end = position;
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

    impl Map {
        fn new(walls: HashSet<Position>, start: Position, end: Position, dimensions: (usize, usize)) -> Self {
            Map { walls, start, end, dimensions }
        }

        fn possible_cheat_with_minimal_save(&self, max_length: isize, min_save: isize) -> usize {
            let mut visited: HashSet<Position> = HashSet::new();

            let mut queue = VecDeque::new();
            queue.push_back((0, self.start));

            let mut result = 0;

            while let Some((distance, current)) = queue.pop_front() {
                if distance > max_length {
                    continue;
                }

                if current == self.end {
                    continue;
                }

                if visited.contains(&current) {
                    continue;
                }

                visited.insert(current);

                for n in self.neighbours(current) {
                    if visited.contains(&n) {
                        continue;
                    }

                    if distance + 1 > max_length {
                        continue;
                    }

                    if !self.walls.contains(&n) {
                        queue.push_back((distance + 1, n));
                        continue;
                    }

                    let mut cheat = 0;

                    let sp = self.shortest_path(visited.clone(), n, max_length as usize);
                    if let Some(s) = sp {
                        if (max_length - (distance + 1 + s as isize)) >= min_save {
                            cheat = 1;
                        }
                    }

                    visited.insert(n);

                    result += cheat;
                }
            }

            result
        }

        fn shortest_path(&self, mut visited: HashSet<Position>, start: Position, max_length: usize) -> Option<usize> {
            let mut queue = VecDeque::new();

            queue.push_back((0, start));

            while let Some((distance, current)) = queue.pop_front() {
                if distance > max_length {
                    return None;
                }

                if current == self.end {
                    return Some(distance);
                }

                if visited.contains(&current) {
                    continue;
                }

                visited.insert(current);

                for n in self.neighbours(current) {
                    if visited.contains(&n) {
                        continue;
                    }

                    if self.walls.contains(&n) {
                        continue;
                    }

                    if distance + 1 > max_length {
                        continue;
                    }

                    queue.push_back((distance + 1, n));
                }
            }

            None
        }

        fn neighbours(&self, (x, y): Position) -> Vec<Position> {
            let mut result = vec![];

            if x > 0 {
                result.push((x - 1, y));
            }

            if x < self.dimensions.0 as isize {
                result.push((x + 1, y));
            }

            if y > 0 {
                result.push((x, y - 1));
            }

            if y < self.dimensions.1 as isize {
                result.push((x, y + 1));
            }


            result
        }
    }

    type Position = (isize, isize);
}