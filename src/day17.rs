#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::ops::BitXor;
    use winnow::stream::ToUsize;

    #[test]
    fn it_execute_instructions() {
        let mut machine = Machine::new(0, 0, 9);
        Bst {}.execute(&mut machine, 6);
        assert_eq!(1, machine.read_registry('B'));

        let mut machine = Machine::new(0, 29, 0);
        Bxl {}.execute(&mut machine, 7);
        assert_eq!(26, machine.read_registry('B'));

        Out {}.execute(&mut machine, 1);
        assert_eq!(vec![1], machine.stdout);

        let mut machine = Machine::new(2024, 0, 0);
        machine.execute_instructions(vec![0, 1, 5, 4, 3, 0]);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", machine.output());
        assert_eq!(0, machine.read_registry('A'));

        let mut machine = Machine::new(729, 0, 0);
        machine.execute_instructions(vec![0, 1, 5, 4, 3, 0]);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", machine.output());
    }

    #[test]
    fn it_parses_input() {
        let input = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
        "};

        assert_eq!("4,6,3,5,6,3,5,2,1,0", execute_program(input));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_day17");

        assert_eq!("6,5,4,7,1,6,0,3,1", execute_program(input));
    }

    fn execute_program(input: &str) -> String {
        let lines = read_lines(input);
        let register_a = lines[0].replace("Register A: ", "").parse::<usize>().unwrap();
        let register_b = lines[1].replace("Register B: ", "").parse::<usize>().unwrap();
        let register_c = lines[2].replace("Register C: ", "").parse::<usize>().unwrap();

        let instruction = lines[4]
            .replace("Program: ", "")
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let mut machine = Machine::new(register_a, register_b, register_c);
        machine.execute_instructions(instruction);
        machine.output()
    }

    // Op 0,6,7
    struct Dv {
        registry: char,
    }

    impl Op for Dv {
        fn execute(&self, machine: &mut Machine, operand: usize) {
            let operand_value = match operand {
                1..=3 => operand,
                4 => machine.read_registry('A'),
                5 => machine.read_registry('B'),
                6 => machine.read_registry('C'),
                _ => panic!("Invalid program")
            };
            let value = machine.read_registry('A') as isize / 2_isize.pow(operand_value as u32);
            machine.write_to_registry(self.registry, value as usize);
        }
    }

    // Op 1
    struct Bxl;

    impl Op for Bxl {
        fn execute(&self, machine: &mut Machine, operand: usize) {
            let value = machine.read_registry('B');
            machine.write_to_registry('B', value.bitxor(operand));
        }
    }

    // Op 2
    struct Bst;

    impl Op for Bst {
        fn execute(&self, machine: &mut Machine, operand: usize) {
            let value = match operand {
                1..=3 => operand,
                4 => machine.read_registry('A'),
                5 => machine.read_registry('B'),
                6 => machine.read_registry('C'),
                _ => panic!("Invalid program")
            };

            machine.write_to_registry('B', value % 8);
        }
    }

    // Op 3
    struct Jnz;

    impl Op for Jnz {
        fn execute(&self, _machine: &mut Machine, _operand: usize) {}

        fn next_instruction(&self, machine: &mut Machine, operand: usize) {
            let value = machine.read_registry('A');
            if value == 0 {
                machine.instruction_pointer += 2;
                return;
            }
            machine.instruction_pointer = operand;
        }
    }

    // Op 4
    struct Bxc;

    impl Op for Bxc {
        fn execute(&self, machine: &mut Machine, _operand: usize) {
            let value = machine.read_registry('B');
            let operand = machine.read_registry('C');
            machine.write_to_registry('B', value.bitxor(operand).to_usize());
        }
    }

    // Op 5
    struct Out;

    impl Op for Out {
        fn execute(&self, machine: &mut Machine, operand: usize) {
            let operand_value = match operand {
                1..=3 => operand,
                4 => machine.read_registry('A'),
                5 => machine.read_registry('B'),
                6 => machine.read_registry('C'),
                _ => panic!("Invalid program")
            };
            machine.write_to_stdout(operand_value % 8);
        }
    }
    struct Machine {
        registries: HashMap<char, usize>,
        stdout: Vec<usize>,
        instruction_pointer: usize,
    }

    impl Machine {
        fn new(registry_a: usize, registry_b: usize, registry_c: usize) -> Self {
            let mut registries = HashMap::new();
            registries.insert('A', registry_a);
            registries.insert('B', registry_b);
            registries.insert('C', registry_c);

            Self {
                registries,
                stdout: vec![],
                instruction_pointer: 0,
            }
        }

        fn execute_instructions(&mut self, instruction: Vec<usize>) {
            while self.instruction_pointer < instruction.len() - 1 {
                let code = instruction[self.instruction_pointer];
                let operand = instruction[self.instruction_pointer + 1];
                let instruction = Self::decode_instruction(code);
                instruction.execute(self, operand);
                instruction.next_instruction(self, operand);
            }
        }

        fn decode_instruction(code: usize) -> Box<dyn Op> {
            match code {
                0 => Box::new(Dv { registry: 'A' }),
                1 => Box::new(Bxl {}),
                2 => Box::new(Bst {}),
                3 => Box::new(Jnz {}),
                4 => Box::new(Bxc {}),
                5 => Box::new(Out {}),
                6 => Box::new(Dv { registry: 'B' }),
                7 => Box::new(Dv { registry: 'C' }),
                _ => panic!("unknown instruction for code {}", code)
            }
        }

        fn output(&self) -> String {
            Itertools::join(&mut self.stdout.iter(), ",")
        }

        fn read_registry(&self, registry: char) -> usize {
            *self.registries.get(&registry).unwrap()
        }

        fn write_to_registry(&mut self, registry: char, value: usize) {
            self.registries.insert(registry, value);
        }

        fn write_to_stdout(&mut self, value: usize) {
            self.stdout.push(value)
        }
    }

    trait Op {
        fn execute(&self, machine: &mut Machine, operand_code: usize);

        fn next_instruction(&self, machine: &mut Machine, _operand: usize) {
            machine.instruction_pointer += 2
        }
    }
}