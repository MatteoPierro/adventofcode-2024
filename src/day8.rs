use crate::input_reader::read_lines;
use std::collections::{HashMap, HashSet};

struct Map {
    map: HashMap<char, Vec<(isize, isize)>>,
    dimension: isize,
}

impl Map {
    fn build_from(raw_input: &str) -> Self {
        let mut map = HashMap::new();

        let input = read_lines(raw_input);
        for (y, line) in input.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }
                let position = (x as isize, y as isize);
                map.entry(char)
                    .and_modify(|positions: &mut Vec<_>| positions.push(position))
                    .or_insert(vec![position]);
            }
        }

        Self { map, dimension: input.len() as isize }
    }

    fn all_antinodes(&self, antinode_extender: AntinodeExtender) -> HashSet<Position> {
        let mut result = HashSet::new();

        for (_, positions) in &self.map {
            for i in 0..(positions.len() - 1) {
                for j in (i + 1)..positions.len() {
                    result.extend(antinode_extender(positions[i], positions[j], self.dimension))
                }
            }
        }

        result
    }
}


type Dimension = isize;
type AntinodeExtender = fn(Position, Position, Dimension) -> HashSet<Position>;
type Position = (isize, isize);

fn antinodes(p1: Position, p2: Position, dimension: isize) -> HashSet<Position> {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;

    let mut result = HashSet::new();

    if is_position_in_boundaries((p1.0 + dx, p1.1 + dy), dimension) {
        result.insert((p1.0 + dx, p1.1 + dy));
    }

    if is_position_in_boundaries((p2.0 - dx, p2.1 - dy), dimension) {
        result.insert((p2.0 - dx, p2.1 - dy));
    }

    result
}

fn antinodes_harmonics(p1: Position, p2: Position, dimension: isize) -> HashSet<Position> {
    let mut result = HashSet::new();

    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;

    let mut current = p1;

    while is_position_in_boundaries(current, dimension) {
        result.insert(current);
        current = (current.0 + dx, current.1 + dy);
    }

    current = p2;

    while is_position_in_boundaries(current, dimension) {
        result.insert(current);
        current = (current.0 - dx, current.1 - dy);
    }

    result
}

fn is_position_in_boundaries(position: (isize, isize), dimension: isize) -> bool {
    position.0 >= 0 && position.0 < dimension && position.1 >= 0 && position.1 < dimension
}

#[cfg(test)]
mod tests {
    use crate::day8::{antinodes, antinodes_harmonics, Map};
    use crate::input_reader::read_input_file;
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn it_calculates_antinodes() {
        let input = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............"};


        assert_eq!(HashSet::from([(3, 1), (6, 7)]), antinodes((4, 3), (5, 5), 10));
        assert_eq!(HashSet::from([(2, 6)]), antinodes((8, 4), (5, 5), 10));

        let map = Map::build_from(input);

        assert_eq!(12, map.dimension);
        assert_eq!(14, map.all_antinodes(antinodes).len());
        assert_eq!(34, map.all_antinodes(antinodes_harmonics).len())
    }

    #[test]
    fn it_solves_both_puzzles() {
        let input = &read_input_file("input_08");

        let map = Map::build_from(input);
        assert_eq!(344, map.all_antinodes(antinodes).len());
        assert_eq!(1182, map.all_antinodes(antinodes_harmonics).len())
    }
}