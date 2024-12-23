#[cfg(test)]
mod tests {
    use crate::input_reader::{read_input_file, read_lines};
    use indoc::indoc;
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn it_counts_combinations_starting_with_t() {
        let input = indoc! {"
            kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn
        "};

        let connections = parse_connections(input);
        let all = combinations(&connections, 3);
        assert_eq!(7, count_combinations_starting_with_t(&all));
        assert_eq!("co,de,ka,ta", find_largest_set_of_computers(&connections));
    }

    #[test]
    fn it_finds_largest_set_of_computers() {
        let input = indoc! {"
            kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn
        "};

        let connections = parse_connections(input);
        assert_eq!("co,de,ka,ta", find_largest_set_of_computers(&connections));
    }

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_23");

        let connections = parse_connections(input);
        let all = combinations(&connections, 3);
        assert_eq!(1184, count_combinations_starting_with_t(&all));
    }

    #[test]
    fn it_solves_second_puzzle() {
        let input = &read_input_file("input_23");

        let connections = parse_connections(input);
        assert_eq!("hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv", find_largest_set_of_computers(&connections));
    }

    fn find_largest_set_of_computers(connections: &HashMap<String, HashSet<String>>) -> String {
        let mut result = vec![];

        for connection in connections.values() {
            let combinations = connection.into_iter()
                .combinations(2)
                .map(|c| connections[c[0]].intersection(&connections[c[1]]).sorted().collect::<Vec<_>>())
                .collect::<HashSet<_>>();

            for combination in combinations {
                let intersection = combination.iter()
                    .map(|&c| connections[c].clone())
                    .reduce(|a, b| a.intersection(&b).map(|v| v.to_string()).collect::<HashSet<_>>())
                    .unwrap();
                let sorted_intersection = intersection
                    .iter()
                    .sorted()
                    .collect_vec();

                if sorted_intersection == combination && sorted_intersection.len() > result.len() {
                    result = combination;
                }
            }
        }
        result.into_iter().join(",")
    }

    fn count_combinations_starting_with_t(all: &HashSet<Vec<String>>) -> i32 {
        let mut result = 0;
        for s in all {
            if s.iter().any(|p| p.starts_with("t")) {
                result += 1;
            }
        }
        result
    }

    fn combinations(connections: &HashMap<String, HashSet<String>>, length: usize) -> HashSet<Vec<String>> {
        let mut all = HashSet::new();
        for (_c1, v) in connections {
            for c2 in v.iter().combinations(length) {
                let set = HashSet::from_iter(c2.clone().iter().map(|s| s.to_string()));
                if c2.iter().all(|&c3| connections[c3].intersection(&set).count() == length) {
                    let vec = set.iter().sorted().map(|v| v.to_string()).collect::<Vec<_>>();
                    all.insert(vec);
                }
            }
        }
        all
    }

    fn parse_connections(input: &str) -> HashMap<String, HashSet<String>> {
        let mut connections = HashMap::new();
        for line in read_lines(input) {
            let parts: Vec<_> = line.split("-").collect();
            let first = parts[0].to_string();
            let second = parts[1].to_string();
            connections.entry(first.clone())
                .and_modify(|v: &mut HashSet<String>| { v.insert(second.clone()); })
                .or_insert(HashSet::from([first.clone(), second.clone()]));
            connections.entry(second.clone())
                .and_modify(|v: &mut HashSet<String>| { v.insert(first.clone()); })
                .or_insert(HashSet::from([second, first]));
        }
        connections
    }
}