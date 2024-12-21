#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use std::collections::{HashMap, HashSet, VecDeque};

    #[test]
    fn it_calculates_the_sum_of_complexity() {
        let input = indoc! {"
            029A
            980A
            179A
            456A
            379A
        "};

        assert_eq!(126384, sum_of_complexity(input, 2))
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_21");

        assert_eq!(237342, sum_of_complexity(input, 2));
        assert_eq!(294585598101704, sum_of_complexity(input, 25));
    }

    fn sum_of_complexity(input: &str, levels: usize) -> usize {
        let numeric_keyboard = HashMap::from([
            ('7', (0, 0)),
            ('8', (1, 0)),
            ('9', (2, 0)),
            ('4', (0, 1)),
            ('5', (1, 1)),
            ('6', (2, 1)),
            ('1', (0, 2)),
            ('2', (1, 2)),
            ('3', (2, 2)),
            ('0', (1, 3)),
            ('A', (2, 3)),
        ]);

        let directional_keypad = HashMap::from([
            ('^', (1, 0)),
            ('A', (2, 0)),
            ('<', (0, 1)),
            ('v', (1, 1)),
            ('>', (2, 1)),
        ]);

        let numeric_keyboard_min_sequences = find_min_sequences_on_keyboard(&numeric_keyboard, neighbours_numeric_keyboard);
        let directional_keymap_min_sequences = find_min_sequences_on_keyboard(&directional_keypad, neighbours_directional_keypad);

        let mut memo = HashMap::new();

        read_lines(input)
            .iter()
            .map(|code| {
                complexity(
                    &numeric_keyboard,
                    &directional_keypad,
                    &numeric_keyboard_min_sequences,
                    &directional_keymap_min_sequences,
                    &mut memo,
                    code,
                    levels)
            }).sum()
    }

    fn complexity(
        numeric_keyboard: &HashMap<char, (isize, isize)>,
        directional_keypad: &HashMap<char, (isize, isize)>,
        numeric_keyboard_min_sequences: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>,
        directional_keymap_min_sequences: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>,
        mut memo: &mut HashMap<(Vec<char>, usize), usize>,
        code: &str,
        levels: usize,
    ) -> usize {
        let length_shortest_sequence = find_length_shortest_sequence(&numeric_keyboard, &directional_keypad, &numeric_keyboard_min_sequences, &directional_keymap_min_sequences, &mut memo, code, levels);
        let numeric_part = code.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
        length_shortest_sequence * numeric_part
    }

    fn find_length_shortest_sequence(numeric_keyboard: &HashMap<char, (isize, isize)>, directional_keypad: &HashMap<char, (isize, isize)>, numeric_keyboard_min_sequences: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>, directional_keymap_min_sequences: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>, mut memo: &mut HashMap<(Vec<char>, usize), usize>, code: &str, levels: usize) -> usize {
        let mut start = 'A';
        let mut length_shortest_sequence = 0;
        for end in code.chars() {
            length_shortest_sequence += move_from_start_to_end(
                &numeric_keyboard,
                &directional_keypad,
                &numeric_keyboard_min_sequences,
                &directional_keymap_min_sequences,
                &end,
                &start,
                &mut memo,
                levels);
            start = end;
        }
        length_shortest_sequence
    }

    fn move_from_start_to_end
    (numeric_keyboard: &HashMap<char, (isize, isize)>,
     directional_keypad: &HashMap<char, (isize, isize)>,
     numeric_keyboard_min_sequences: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>,
     directional_keymap_min_sequences: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>,
     end: &char,
     start: &char,
     memo: &mut HashMap<(Vec<char>, usize), usize>,
     levels: usize,
    ) -> usize {
        let sequences = numeric_keyboard_min_sequences[&(numeric_keyboard[&start], numeric_keyboard[&end].clone())].clone();
        let mut result = usize::MAX;
        for sequence in sequences {
            let r = sequence_length(sequence, levels, &directional_keypad, memo, &directional_keymap_min_sequences);
            if r < result {
                result = r;
            }
        }
        result
    }

    fn sequence_length(
        sequence: Vec<char>,
        level: usize,
        keymap_sequences: &HashMap<char, (isize, isize)>,
        memo: &mut HashMap<(Vec<char>, usize), usize>,
        cells_connections: &HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>>,
    ) -> usize {
        if level == 0 {
            return sequence.len();
        }

        if let Some(result) = memo.get(&(sequence.clone(), level)) {
            return *result;
        }

        let mut start = keymap_sequences[&'A'].clone();

        let mut total = 0;
        for e in &sequence {
            let end = keymap_sequences[&e];
            let mut min = usize::MAX;
            for s in &cells_connections[&(start.clone(), end.clone())] {
                let c = sequence_length(s.clone(), level - 1, keymap_sequences, memo, cells_connections);
                if c < min {
                    min = c;
                }
            }
            total += min;
            start = end;
        }

        memo.insert((sequence.clone(), level), total);
        total
    }

    fn find_min_sequences_on_keyboard(keyboard: &HashMap<char, (isize, isize)>, neighbour: fn((isize, isize)) -> Vec<((isize, isize), char)>) -> HashMap<((isize, isize), (isize, isize)), Vec<Vec<char>>> {
        let mut sequence = HashMap::new();

        for &s in keyboard.values() {
            for &e in keyboard.values() {
                let distance = find_sequence(s, e, neighbour);
                sequence.insert((s, e), distance);
            }
        }

        sequence
    }

    fn find_sequence(
        start_position: (isize, isize),
        end_position: (isize, isize),
        neighbour: fn((isize, isize)) -> Vec<((isize, isize), char)>,
    ) -> Vec<Vec<char>> {
        let current_position = start_position;
        let mut sequences = vec![];
        let mut queue = VecDeque::from([(current_position, 0, vec![], HashSet::new())]);
        let mut min_length = usize::MAX;

        while let Some((p, index, s, visited)) = queue.pop_front() {
            if s.len() > min_length {
                continue;
            }

            if p == end_position {
                let mut copy = s.clone();
                copy.push('A');
                min_length = copy.len();
                sequences.push(copy);
                continue;
            }

            for (n, step) in neighbour(p) {
                if visited.contains(&n) {
                    continue;
                }
                let mut c = s.clone();
                c.push(step);
                let mut vc = visited.clone();
                vc.insert(p);
                queue.push_back((n, index, c, vc))
            }
        }

        sequences
    }

    fn neighbours_directional_keypad((x, y): (isize, isize)) -> Vec<((isize, isize), char)> {
        if (x, y) == (1, 0) {
            return vec![
                ((2, 0), '>'),
                ((1, 1), 'v'),
            ];
        }

        if (x, y) == (0, 1) {
            return vec![
                ((1, 1), '>')
            ];
        }

        let mut result = vec![];

        if x != 0 {
            result.push(((x - 1, y), '<'))
        }

        if x != 2 {
            result.push(((x + 1, y), '>'))
        }

        if y != 0 {
            result.push(((x, y - 1), '^'))
        }

        if y != 1 {
            result.push(((x, y + 1), 'v'))
        }

        result
    }

    fn neighbours_numeric_keyboard((x, y): (isize, isize)) -> Vec<((isize, isize), char)> {
        if (x, y) == (1, 3) {
            return vec![
                ((x, y - 1), '^'),
                ((x + 1, y), '>')
            ];
        }

        if (x, y) == (0, 2) {
            return vec![
                ((0, 1), '^'),
                ((1, 2), '>')
            ];
        }

        let mut result = vec![];

        if x != 0 {
            result.push(((x - 1, y), '<'));
        }

        if x != 2 {
            result.push(((x + 1, y), '>'));
        }

        if y != 0 {
            result.push(((x, y - 1), '^'));
        }

        if y != 3 {
            result.push(((x, y + 1), 'v'));
        }

        result
    }
}