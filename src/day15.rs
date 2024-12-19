#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use num::range;
    use std::collections::HashSet;

    #[test]
    fn it_executes_instructions() {
        let input = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
        "};

        let (instructions, mut map) = parse_input(input, read_map);
        map.execute(instructions);
        map.show();
    }

    #[test]
    fn it_sums_all_gps_coordinates() {
        let input = &read_input_file("test_input_15");

        assert_eq!(10092, sum_all_gps_coordinates(input));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_15");

        assert_eq!(1475249, sum_all_gps_coordinates(input));
    }

    #[test]
    fn it_sums_all_gps_coordinates_after_expansion_and_execution() {
        let input = &read_input_file("test_input_15");

        let (instructions, mut map) = parse_input(input, read_doubled_map);
        map.show();
        map.execute(instructions);
        map.show();

        assert_eq!(9021, map.sum_all_gps_coordinates())
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_15");
        let (instructions, mut map) = parse_input(input, read_doubled_map);
        map.execute(instructions);

        assert_eq!(1509724, map.sum_all_gps_coordinates());
    }

    fn sum_all_gps_coordinates(input: &String) -> usize {
        let (instructions, mut map) = parse_input(input, read_map);
        map.execute(instructions);
        map.sum_all_gps_coordinates()
    }

    fn parse_input<T>(input: &str, map_parser: fn(Vec<String>) -> Map<T>) -> (Vec<char>, Map<T>) {
        let lines = read_lines(input);

        let mut raw_map = vec![];
        let mut index = 0;
        while !lines[index].is_empty() {
            raw_map.push(lines[index].clone());
            index += 1;
        }

        let mut instructions: Vec<char> = vec![];
        for i in index..lines.len() {
            let mut instructions_in_line: Vec<char> = lines[i].chars().collect();
            instructions.append(&mut instructions_in_line);
        }


        (instructions, map_parser(raw_map))
    }

    fn read_map(raw_map: Vec<String>) -> Map<SimpleBox> {
        let mut walls = HashSet::new();
        let mut robot = (0, 0);
        let mut boxes = HashSet::new();
        for (y, line) in raw_map.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    walls.insert((x, y));
                }

                if char == '@' {
                    robot = (x, y);
                }

                if char == 'O' {
                    boxes.insert(SimpleBox((x, y)));
                }
            }
        }

        let dimensions = (raw_map.len(), raw_map[0].len());
        Map { walls, robot, boxes, dimensions }
    }

    fn read_doubled_map(raw_map: Vec<String>) -> Map<DoubleBox> {
        let mut walls = HashSet::new();
        let mut robot = (0, 0);
        let mut boxes = HashSet::new();
        for (y, line) in raw_map.iter().enumerate() {
            let mut x = 0;
            for char in line.chars() {
                if char == '#' {
                    walls.insert((x, y));
                    walls.insert((x + 1, y));
                }

                if char == '@' {
                    robot = (x, y);
                }

                if char == 'O' {
                    boxes.insert(DoubleBox { start: (x, y), end: (x + 1, y) });
                }
                x += 2;
            }
        }

        let dimensions = (raw_map[0].len() * 2, raw_map.len());
        Map { walls, robot, boxes, dimensions }
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    struct DoubleBox {
        start: Position,
        end: Position,
    }

    impl DoubleBox {
        fn move_to(&self, instruction: char) -> Self {
            let (sx, sy) = self.start;
            let (ex, ey) = self.end;
            match instruction {
                '^' => DoubleBox { start: (sx, sy - 1), end: (ex, ey - 1) },
                'v' => DoubleBox { start: (sx, sy + 1), end: (ex, ey + 1) },
                '<' => DoubleBox { start: (sx - 1, sy), end: (sx, sy) },
                '>' => DoubleBox { start: (ex, sy), end: (ex + 1, sy) },
                _ => panic!("invalid instruction")
            }
        }

        fn neighbours(&self, instruction: char) -> HashSet<Self> {
            let (sx, sy) = self.start;
            let (ex, ey) = self.end;
            match instruction {
                '^' => HashSet::from([
                    DoubleBox { start: (sx - 1, sy - 1), end: (sx, ey - 1) },
                    DoubleBox { start: (sx, sy - 1), end: (ex, ey - 1) },
                    DoubleBox { start: (ex, sy - 1), end: (ex + 1, ey - 1) },
                ]),
                'v' => HashSet::from([
                    DoubleBox { start: (sx - 1, sy + 1), end: (sx, ey + 1) },
                    DoubleBox { start: (sx, sy + 1), end: (ex, ey + 1) },
                    DoubleBox { start: (ex, sy + 1), end: (ex + 1, ey + 1) },
                ]),
                '<' => HashSet::from([
                    DoubleBox { start: (sx - 2, sy), end: (sx - 1, sy) },
                ]),
                '>' => HashSet::from([
                    DoubleBox { start: (ex + 1, sy), end: (ex + 2, sy) },
                ]),
                _ => panic!("invalid instruction")
            }
        }
    }

    impl Map<DoubleBox> {
        fn execute(&mut self, instructions: Vec<char>) {
            for instruction in instructions {
                self.execute_instruction(instruction)
            }
        }

        fn sum_all_gps_coordinates(&self) -> usize {
            let mut result = 0;
            for b in &self.boxes {
                result += b.start.1 * 100 + b.start.0;
            }
            result
        }

        fn boxes_for_position_and_direction((x, y): Position, instruction: char) -> HashSet<DoubleBox> {
            match instruction {
                '^' => HashSet::from([
                    DoubleBox { start: (x - 1, y), end: (x, y) },
                    DoubleBox { start: (x, y), end: (x + 1, y) }
                ]),
                'v' => HashSet::from([
                    DoubleBox { start: (x - 1, y), end: (x, y) },
                    DoubleBox { start: (x, y), end: (x + 1, y) }
                ]),
                '<' => HashSet::from([
                    DoubleBox { start: (x - 1, y), end: (x, y) }
                ]),
                '>' => HashSet::from([
                    DoubleBox { start: (x, y), end: (x + 1, y) }
                ]),
                _ => panic!("invalid instruction")
            }
        }

        fn execute_instruction(&mut self, instruction: char) {
            let next_robot_position = robot_step(&self.robot, instruction);
            if self.walls.contains(&next_robot_position) {
                return;
            }

            let mut current_boxes = Self::boxes_for_position_and_direction(next_robot_position, instruction);
            if self.boxes.intersection(&current_boxes).next().is_none() {
                self.robot = next_robot_position;
                return;
            }

            let mut boxes_to_add = HashSet::new();
            let mut boxes_to_remove = HashSet::new();

            while self.boxes.intersection(&current_boxes).next().is_some() {
                boxes_to_remove.extend(current_boxes.clone());

                let mut next_boxes = HashSet::new();
                let mut next_positions = HashSet::new();

                for b in self.boxes.intersection(&current_boxes) {
                    let bxs = b.move_to(instruction);
                    next_positions.insert(bxs.start);
                    next_positions.insert(bxs.end);
                    boxes_to_add.insert(bxs);
                    let neighbours = b.neighbours(instruction);
                    for n in neighbours {
                        if !self.boxes.contains(&n) {
                            continue;
                        }
                        next_boxes.insert(n);
                    }
                }

                if self.walls.intersection(&next_positions).next().is_some() {
                    boxes_to_remove.clear();
                    break;
                }

                current_boxes = next_boxes;
            }

            if !boxes_to_remove.is_empty() {
                for b in boxes_to_remove {
                    self.boxes.remove(&b);
                }
                self.boxes.extend(boxes_to_add);
                self.robot = next_robot_position;
            }
        }
        fn show(&self) {
            for y in range(0, self.dimensions.1) {
                let mut x = 0;
                while x < self.dimensions.0 {
                    if self.walls.contains(&(x, y)) {
                        print!("#");
                        x += 1;
                        continue;
                    }

                    if self.robot == (x, y) {
                        print!("@");
                        x += 1;
                        continue;
                    }

                    if self.boxes.contains(&DoubleBox { start: (x, y), end: (x + 1, y) }) {
                        print!("[]");
                        x += 2;
                        continue;
                    }

                    print!(".");
                    x += 1;
                }
                println!();
            }
        }
    }

    #[derive(Debug)]
    struct Map<T> {
        walls: HashSet<Position>,
        robot: Position,
        boxes: HashSet<T>,
        dimensions: (usize, usize),
    }

    fn robot_step((x, y): &Position, instruction: char) -> Position {
        match instruction {
            '<' => (x - 1, *y),
            '>' => (x + 1, *y),
            '^' => (*x, y - 1),
            'v' => (*x, y + 1),
            _ => panic!("unknown instruction")
        }
    }

    impl Map<SimpleBox> {
        fn execute(&mut self, instructions: Vec<char>) {
            for instruction in instructions {
                self.execute_instruction(instruction)
            }
        }

        fn execute_instruction(&mut self, instruction: char) {
            let next_robot_position = robot_step(&self.robot, instruction);
            if self.walls.contains(&next_robot_position) {
                return;
            }

            if !self.boxes.contains(&SimpleBox(next_robot_position.clone())) {
                self.robot = next_robot_position;
                return;
            }

            let mut current_box = SimpleBox(next_robot_position.clone());
            let mut box_to_remove = Some(current_box.clone());
            while self.boxes.contains(&current_box) {
                current_box = current_box.move_to(instruction);
                if self.walls.contains(&current_box.0) {
                    box_to_remove = None;
                }
            }

            if let Some(b) = box_to_remove {
                self.boxes.remove(&b);
                self.boxes.insert(current_box);
                self.robot = next_robot_position;
            }
        }
        fn show(&self) {
            for y in range(0, self.dimensions.1) {
                for x in range(0, self.dimensions.0) {
                    if self.walls.contains(&(x, y)) {
                        print!("#");
                        continue;
                    }

                    if self.robot == (x, y) {
                        print!("@");
                        continue;
                    }

                    if self.boxes.contains(&SimpleBox((x, y))) {
                        print!("O");
                        continue;
                    }

                    print!(".");
                }
                println!();
            }
        }

        fn sum_all_gps_coordinates(&self) -> usize {
            self.boxes.iter()
                .map(|b| b.box_gps_coordinates())
                .sum()
        }
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    struct SimpleBox(Position);

    impl SimpleBox {
        fn move_to(&self, instruction: char) -> Self {
            let (x, y) = self.0;
            match instruction {
                '<' => SimpleBox((x - 1, y)),
                '>' => SimpleBox((x + 1, y)),
                '^' => SimpleBox((x, y - 1)),
                'v' => SimpleBox((x, y + 1)),
                _ => panic!("unknown instruction")
            }
        }

        fn box_gps_coordinates(&self) -> usize {
            let (x, y) = self.0;
            100 * y + x
        }
    }

    type Position = (usize, usize);
}