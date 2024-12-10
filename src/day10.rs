use crate::input_reader::read_lines;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Eq, PartialEq)]
struct Position(isize, isize);

impl Position {
    fn neighbours_position(&self, dimension: isize) -> Vec<Position> {
        [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0)
        ].iter()
            .filter_map(|(dx, dy)| {
                if self.0 + dx < 0 || self.0 + dx >= dimension {
                    return None;
                }

                if self.1 + dy < 0 || self.1 + dy >= dimension {
                    return None;
                }

                Some(Position(self.0 + dx, self.1 + dy))
            }).collect()
    }
}

struct Map {
    map: Vec<Vec<char>>,
    zeros: Vec<Position>,
    dimension: isize,
}

impl Map {
    fn build_from(input: &str) -> Self {
        let mut zeros = vec![];
        let mut map = vec![];

        for (y, line) in read_lines(input).iter().enumerate() {
            map.push(line.chars().collect::<Vec<_>>());
            for (x, c) in line.chars().enumerate() {
                if c == '0' {
                    zeros.push(Position(x as isize, y as isize))
                }
            }
        }

        let dimension = map.len() as isize;

        Map { zeros, map, dimension }
    }

    fn find_nines(&self) -> usize {
        self.zeros
            .iter()
            .map(|z| {
                let mut nines = HashSet::new();
                self.find_nines_from_position(z, &mut nines);
                nines.len()
            })
            .sum()
    }

    fn find_nines_from_position(&self, position: &Position, nines: &mut HashSet<Position>) {
        if let Some(9) = self.value_at_position(&position) {
            nines.insert(position.clone());
            return;
        }

        let current = self.value_at_position(&position).unwrap();

        for np in position.neighbours_position(self.dimension) {
            let n = self.value_at_position(&np);
            if n.is_none() {
                continue;
            }

            if n.unwrap() != current + 1 {
                continue;
            }

            self.find_nines_from_position(&np, nines);
        }
    }

    fn find_all_paths(&self) -> usize {
        self.zeros
            .iter()
            .map(|z| self.find_all_paths_from_position(z))
            .sum()
    }

    fn find_all_paths_from_position(&self, position: &Position) -> usize {
        if let Some(9) = self.value_at_position(&position) {
            return 1;
        }

        let current = self.value_at_position(&position).unwrap();

        position.neighbours_position(self.dimension)
            .iter()
            .filter_map(|np| {
                let n = self.value_at_position(&np);
                if n.is_none() {
                    return None;
                }
                if n.unwrap() != current + 1 {
                    return None;
                }

                Some(self.find_all_paths_from_position(&np))
            }).sum()
    }

    fn value_at_position(&self, Position(x, y): &Position) -> Option<usize> {
        self.map[*y as usize][*x as usize].to_digit(10).map(|v| v as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::Map;
    use crate::input_reader::read_input_file;
    use indoc::indoc;

    #[test]
    fn it_finds_score() {
        assert_eq!(2, find_score(indoc! {"
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9"}));

        assert_eq!(36, find_score(indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"}));
    }

    #[test]
    fn it_finds_all_paths_score() {
        assert_eq!(3, find_all_paths_score(indoc! {"
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9...."}));

        assert_eq!(81, find_all_paths_score(indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"}));
    }

    #[test]
    fn it_solves_both_puzzles() {
        let input = &read_input_file("input_10");

        assert_eq!(489, find_score(input));
        assert_eq!(1086, find_all_paths_score(input))
    }

    fn find_score(input: &str) -> usize {
        Map::build_from(input).find_nines()
    }

    fn find_all_paths_score(input: &str) -> usize {
        Map::build_from(input).find_all_paths()
    }
}