#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use priority_queue::PriorityQueue;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn it_find_the_lowest_score() {
        let input = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
        "};

        let (walls, start, end) = parse_map(input);
        assert_eq!(7036, find_lowest_score(&walls, start, end));
        assert_eq!(45, tiles_on_best_paths(&walls, start, end, 7036).len());

        let input = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
        "};

        let (walls, start, end) = parse_map(input);
        assert_eq!(11048, find_lowest_score(&walls, start, end));
        assert_eq!(64, tiles_on_best_paths(&walls, start, end, 11048).len());
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_16");
        let (walls, start, end) = parse_map(input);
        assert_eq!(91464, find_lowest_score(&walls, start, end));
        assert_eq!(494, tiles_on_best_paths(&walls, start, end, 91464).len()); // slow ~ 1 minute
    }

    fn parse_map(input: &str) -> (HashSet<(isize, isize)>, (isize, isize), (isize, isize)) {
        let mut walls = HashSet::new();
        let mut start = (-1, -1);
        let mut end = (-1, -1);
        for (y, line) in read_lines(input).iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    walls.insert((x as isize, y as isize));
                }

                if char == 'S' {
                    start = (x as isize, y as isize)
                }

                if char == 'E' {
                    end = (x as isize, y as isize)
                }
            }
        }
        (walls, start, end)
    }

    fn find_lowest_score(walls: &HashSet<(isize, isize)>, start: (isize, isize), end: (isize, isize)) -> isize {
        let mut pq = PriorityQueue::new();
        pq.push((start, Direction::East, vec![]), 0);
        let mut final_score = 0;

        let mut visited = HashSet::new();
        visited.insert((start, Direction::East));

        while let Some(((position, orientation, path), score)) = pq.pop() {
            if position == end {
                final_score = score;
                break;
            }

            for (p, d, s) in neighbours(&position, &orientation) {
                if visited.contains(&(p.clone(), d.clone())) || walls.contains(&p) {
                    continue;
                }

                visited.insert((p.clone(), d.clone()));
                let mut new_path = path.clone();
                new_path.push((p.clone(), d.clone()));
                pq.push((p, d, new_path), score + s);
            }
        }

        final_score * -1
    }

    fn tiles_on_best_paths(walls: &HashSet<(isize, isize)>, start_position: Position, end_position: Position, credit: isize) -> HashSet<Position> {
        let mut queue = PriorityQueue::new();
        let current = vec![start_position];
        queue.push((start_position, Direction::East, current), 0);
        let mut seen = HashMap::new();
        let mut best = HashSet::new();

        while let Some(((location, direction, path), score)) = queue.pop() {
            seen.insert((location, direction), score);

            if location == end_position {
                best.extend(path);
                continue;
            }

            for (n, d, cost) in neighbours(&location, &direction) {
                if walls.contains(&n) {
                    continue;
                }

                if -(score + cost) > credit {
                    continue;
                }

                if *seen.get(&(n, d)).unwrap_or(&isize::MIN) > (score + cost) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(n);
                queue.push((n, d, new_path), score + cost);
            }
        }

        best
    }

    type Position = (isize, isize);

    #[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    fn neighbours((x, y): &Position, direction: &Direction) -> Vec<(Position, Direction, isize)> {
        if *direction == Direction::East {
            return vec![
                ((x + 1, *y), Direction::East, -1),
                ((*x, y - 1), Direction::North, -1001),
                ((*x, y + 1), Direction::South, -1001),
            ];
        }

        if *direction == Direction::West {
            return vec![
                ((x - 1, *y), Direction::West, -1),
                ((*x, y - 1), Direction::North, -1001),
                ((*x, y + 1), Direction::South, -1001),
            ];
        }

        if *direction == Direction::North {
            return vec![
                ((*x, y - 1), Direction::North, -1),
                ((x - 1, *y), Direction::West, -1001),
                ((x + 1, *y), Direction::East, -1001),
            ];
        }

        vec![
            ((*x, y + 1), Direction::South, -1),
            ((x - 1, *y), Direction::West, -1001),
            ((x + 1, *y), Direction::East, -1001),
        ]
    }
}