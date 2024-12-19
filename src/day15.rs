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

        let (instructions, mut map) = parse_input(input);
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

    fn sum_all_gps_coordinates(input: &String) -> usize {
        let (instructions, mut map) = parse_input(input);
        map.execute(instructions);

        let mut result = 0;
        for b in map.boxes {
            let (x, y) = b.0;
            result += 100 * y + x;
        }
        result
    }

    fn parse_input(input: &str) -> (Vec<char>, Map) {
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


        let map = read_map(raw_map);
        (instructions, map)
    }

    fn read_map(raw_map: Vec<String>) -> Map {
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
                    boxes.insert(Box((x, y)));
                }
            }
        }

        let dimensions = (raw_map.len(), raw_map[0].len());
        Map { walls, robot, boxes, dimensions }
    }

    #[derive(Debug)]
    struct Map {
        walls: HashSet<Position>,
        robot: Position,
        boxes: HashSet<Box>,
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

    impl Map {
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

            if !self.boxes.contains(&Box(next_robot_position.clone())) {
                self.robot = next_robot_position;
                return;
            }

            let mut current_box = Box(next_robot_position.clone());
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

                    if self.boxes.contains(&Box((x, y))) {
                        print!("O");
                        continue;
                    }

                    print!(".");
                }
                println!();
            }
        }
    }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    struct Box(Position);

    impl Box {
        fn move_to(&self, instruction: char) -> Self {
            let (x, y) = self.0;
            match instruction {
                '<' => Box((x - 1, y)),
                '>' => Box((x + 1, y)),
                '^' => Box((x, y - 1)),
                'v' => Box((x, y + 1)),
                _ => panic!("unknown instruction")
            }
        }
    }

    type Position = (usize, usize);
}