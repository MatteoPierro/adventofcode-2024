#[cfg(test)]
mod tests {
    use crate::input_reader::read_input_file;

    #[test]
    fn it_expand_the_number() {
        let input = "2333133121414131402";
        let expanded_sequence = expand(input);
        assert_eq!(
            vec!["0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3", "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".", "7", "7", "7", ".", "8", "8", "8", "8", "9", "9"],
            expanded_sequence
        );

        let adjusted = adjust_sequence_contiguous(expanded_sequence);
        assert_eq!(
            vec!["0", "0", "9", "9", "2", "1", "1", "1", "7", "7", "7", ".", "4", "4", ".", "3", "3", "3", ".", ".", ".", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".", ".", ".", ".", ".", "8", "8", "8", "8", ".", "."],
            adjusted);

        // let adjusted = adjust_sequence(expanded_sequence);
        // assert_eq!(
        //     vec!["0", "0", "9", "9", "8", "1", "1", "1", "8", "8", "8", "2", "7", "7", "7", "3", "3", "3", "6", "4", "4", "6", "5", "5", "5", "5", "6", "6", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        //     adjusted);
        assert_eq!(2858, calculate_checksum(adjusted));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_09");

        let expanded_sequence = expand(input);
        let adjusted = adjust_sequence(expanded_sequence);
        assert_eq!(6384282079460, calculate_checksum(adjusted));
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_09");

        let expanded_sequence = expand(input);
        let adjusted = adjust_sequence_contiguous(expanded_sequence);
        assert_eq!(6409057127686, calculate_checksum(adjusted)); // 6409057127686 Too high
    }

    fn calculate_checksum(adjusted: Vec<String>) -> usize {
        let mut result = 0;

        for (index, value) in adjusted.iter().enumerate() {
            if value == "." {
                continue;
            }

            result += index * value.parse::<usize>().unwrap();
        }

        result
    }

    fn adjust_sequence(sequence: Vec<String>) -> Vec<String> {
        let mut adjusted = sequence.clone();
        let mut left = 0;
        let mut right = adjusted.len() - 1;

        loop {
            while adjusted[left] != "." && left < adjusted.len() {
                left += 1;
            }

            while adjusted[right] == "." && right > 0 {
                right -= 1
            }

            if left > right {
                break;
            }

            adjusted.swap(left, right);
        }
        adjusted
    }

    fn adjust_sequence_contiguous(sequence: Vec<String>) -> Vec<String> {
        let mut adjusted = sequence.clone();
        let last = sequence.last().unwrap().parse::<usize>().unwrap();

        for current in (0..=last).rev() {
            let mut right = 0;
            while adjusted[right] != current.to_string() {
                right += 1;
            }

            let mut len = 0;

            while right + len < adjusted.len() && adjusted[right + len] == current.to_string() {
                len += 1
            }

            let mut left = 0;

            while left < right {
                while adjusted[left] != "." {
                    left += 1;
                }

                let mut spaces = 0;

                while left + spaces < adjusted.len() && adjusted[left + spaces] == "." {
                    spaces += 1;
                }

                if spaces >= len {
                    for i in 0..len {
                        adjusted.swap(left + i, right + i);
                    }
                    break;
                }

                left += spaces + 1;
            }
        }

        adjusted
    }

    fn expand(input: &str) -> Vec<String> {
        let mut result = vec![];

        let numbers: Vec<_> = input.chars().collect();
        let mut i = 0;
        let mut current = 0;
        while i < numbers.len() - 1 {
            let times = numbers[i].to_digit(10).unwrap() as usize;
            (0..times).for_each(|_| result.push(current.to_string()));

            let times = numbers[i + 1].to_digit(10).unwrap() as usize;
            (0..times).for_each(|_| result.push(".".to_string()));
            current += 1;
            i += 2;
        }
        let times = numbers[i].to_digit(10).unwrap() as usize;
        (0..times).for_each(|_| result.push(current.to_string()));

        result
    }
}