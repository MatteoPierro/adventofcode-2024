#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashMap, HashSet};

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
            for n in neighbours(&position) {
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
                for n in neighbours(&current) {
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

    fn neighbours((x, y): &(isize, isize)) -> Vec<(isize, isize)> {
        [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1)
        ].iter().map(|(dx, dy)| (x + dx, y + dy))
            .collect()
    }

    fn parse_tiles_positions(input: &str) -> HashMap<char, HashSet<(isize, isize)>> {
        let lines = read_lines(input);

        let mut tiles_positions: HashMap<char, HashSet<(isize, isize)>> = HashMap::new();

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