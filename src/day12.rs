#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashMap, HashSet};
    use Direction::{Down, Left, Right, Up};

    #[test]
    fn it_calculate_total_price() {
        assert_eq!(
            772,
            total_price(
                indoc! {"
                OOOOO
                OXOXO
                OOOOO
                OXOXO
                OOOOO
                "})
        );

        assert_eq!(
            1930,
            total_price(
                indoc! {"
                RRRRIICCFF
                RRRRIICCCF
                VVRRRCCFFF
                VVRCCCJFFF
                VVVVCJJCFE
                VVIVCCJJEE
                VVIIICJJEE
                MIIIIIJJEE
                MIIISIJEEE
                MMMISSJEEE
                "})
        );
    }

    #[test]
    fn it_solves_first_puzzle() {
        assert_eq!(1533644, total_price(&read_input_file("input_12")));
    }

    #[test]
    fn it_calculates_total_price_with_sides() {
        assert_eq!(236,
                   total_price_with_sides(indoc! {"
                           EEEEE
                           EXXXX
                           EEEEE
                           EXXXX
                           EEEEE
                           "}));
    }

    #[test]
    fn it_solves_second_puzzle() {
        assert_eq!(936718,
                   total_price_with_sides(&read_input_file("input_12")));
    }

    // Part 2

    fn total_price_with_sides(input: &str) -> usize {
        parse_tiles_positions(input)
            .values()
            .map(|tile_positions| tile_price_with_sides(tile_positions))
            .sum::<usize>()
    }

    fn tile_price_with_sides(tile_positions: &HashSet<(isize, isize)>) -> usize {
        find_groups(&tile_positions)
            .iter()
            .map(|g| group_price_with_sides(&g))
            .sum::<usize>()
    }

    fn group_price_with_sides(group: &Vec<(isize, isize)>) -> usize {
        find_sides(group) * group.len()
    }

    fn find_sides(group: &Vec<(isize, isize)>) -> usize {
        let mut sides = 0;

        for s in find_segments(group).values_mut() {
            while !s.is_empty() {
                sides += 1;
                let value = s.iter().next().unwrap().clone();
                s.remove(&value);
                let mut i = 1;
                while s.remove(&(value + i)) {
                    i += 1;
                }

                i = 1;
                while s.remove(&(value - i)) {
                    i += 1;
                }
            }
        }

        sides
    }

    fn find_segments(group: &Vec<(isize, isize)>) -> HashMap<(isize, Direction), HashSet<isize>> {
        let mut segments: HashMap<(isize, Direction), HashSet<isize>> = HashMap::new();

        for ((x, y), direction) in group_neighbours(group) {
            let (key, value) = if direction == Up || direction == Down {
                (y, x)
            } else {
                (x, y)
            };

            segments.entry((key, direction))
                .and_modify(|p| { p.insert(value); })
                .or_insert(HashSet::from_iter(vec![value]));
        }

        segments
    }

    fn group_neighbours(group: &Vec<(isize, isize)>) -> HashSet<((isize, isize), Direction)> {
        let mut neighbours = HashSet::new();

        for position in group {
            for n in find_neighbours_with_directions(position) {
                if group.contains(&n.0) {
                    continue;
                }
                neighbours.insert(n);
            }
        }

        neighbours
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn find_neighbours_with_directions((x, y): &(isize, isize)) -> Vec<((isize, isize), Direction)> {
        [
            ((-1, 0), Left),
            ((1, 0), Right),
            ((0, -1), Up),
            ((0, 1), Down)
        ].iter()
            .map(|((dx, dy), direction)| ((x + dx, y + dy), *direction))
            .collect()
    }

    // Part1

    fn total_price(input: &str) -> usize {
        parse_tiles_positions(input)
            .values()
            .map(|tile_positions| tile_price(tile_positions))
            .sum::<usize>()
    }

    fn tile_price(tile_positions: &HashSet<(isize, isize)>) -> usize {
        find_groups(&tile_positions)
            .iter()
            .map(|g| group_price(&tile_positions, &g))
            .sum::<usize>()
    }

    fn group_price(tile_positions: &HashSet<(isize, isize)>, group: &Vec<(isize, isize)>) -> usize {
        let area = group.len();
        let mut perimiter = 0;

        for position in group {
            perimiter += 4;
            for n in find_neighbours(&position) {
                if tile_positions.contains(&n) {
                    perimiter -= 1;
                }
            }
        }

        area * perimiter
    }

    fn find_groups(tile_positions: &HashSet<(isize, isize)>) -> Vec<Vec<(isize, isize)>> {
        let mut groups = vec![];

        let mut copy = tile_positions.clone();
        while !copy.is_empty() {
            let head = copy.iter().next().unwrap().clone();
            let mut to_visit = vec![head.clone()];
            copy.remove(&head);
            let mut group = vec![head];
            while !to_visit.is_empty() {
                let current = to_visit.pop().unwrap();
                for n in find_neighbours(&current) {
                    if !copy.contains(&n) {
                        continue;
                    }

                    copy.remove(&n);
                    group.push(n);
                    to_visit.push(n);
                }
            }
            groups.push(group);
        }

        groups
    }

    fn find_neighbours((x, y): &(isize, isize)) -> Vec<(isize, isize)> {
        [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1)
        ].iter().map(|(dx, dy)| (x + dx, y + dy))
            .collect()
    }

    fn parse_tiles_positions(input: &str) -> HashMap<char, HashSet<(isize, isize)>> {
        let mut tiles_positions: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();

        let lines = read_lines(input);
        for (y, line) in lines.iter().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                tiles_positions.entry(tile)
                    .and_modify(|p| { p.insert((x as isize, y as isize)); })
                    .or_insert(HashSet::from_iter(vec![(x as isize, y as isize)]));
            }
        }

        tiles_positions
    }
}