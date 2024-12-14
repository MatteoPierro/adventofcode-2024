#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use regex::Regex;

    #[test]
    fn it_calculates_safety_factor() {
        let input = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
        "};

        assert_eq!(12, calculate_safety_factor(input, (11, 7)));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_14");

        assert_eq!(221655456, calculate_safety_factor(input, (101, 103)));
    }

    fn calculate_safety_factor(input: &str, dimensions: (isize, isize)) -> usize {
        let mut first = 0;
        let mut second = 0;
        let mut third = 0;
        let mut forth = 0;

        for (position, speed) in parse_robots(input) {
            let (x, y) = move_robot(position, speed, dimensions, 100);

            if x < dimensions.0 / 2 && y < dimensions.1 / 2 {
                first += 1;
                continue;
            }

            if x != dimensions.0 / 2 && y < dimensions.1 / 2 {
                second += 1;
                continue;
            }

            if x < dimensions.0 / 2 && y != dimensions.1 / 2 {
                third += 1;
                continue;
            }

            if x != dimensions.0 / 2 && y != dimensions.1 / 2 {
                forth += 1;
            }
        }

        let safety_factor = first * second * third * forth;
        safety_factor
    }

    fn parse_robots(input: &str) -> Vec<((isize, isize), (isize, isize))> {
        let pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

        let robots: Vec<_> = read_lines(input).iter().map(|r| parse_robot(&pattern, r)).collect();
        robots
    }

    fn parse_robot(pattern: &Regex, robot: &String) -> ((isize, isize), (isize, isize)) {
        pattern
            .captures_iter(robot)
            .map(|c| c.extract())
            .map(|(_, [x, y, vx, vy])| {
                (
                    (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()),
                    (vx.parse::<isize>().unwrap(), vy.parse::<isize>().unwrap())
                )
            }).next().unwrap()
    }

    fn move_robot(robot: (isize, isize), speed: (isize, isize), dimensions: (isize, isize), times: isize) -> (isize, isize) {
        let new_x = (robot.0 + times * speed.0).rem_euclid(dimensions.0);
        let new_y = (robot.1 + times * speed.1).rem_euclid(dimensions.1);
        (new_x, new_y)
    }
}