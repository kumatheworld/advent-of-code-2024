use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

fn build_graph(input: &str) -> HashMap<[u8; 2], HashSet<[u8; 2]>> {
    let mut graph = HashMap::<[u8; 2], HashSet<[u8; 2]>>::new();
    for line in input.lines() {
        let (u, v) = line.split_once('-').unwrap();
        let u = <[u8; 2]>::try_from(u.as_bytes()).unwrap();
        let v = <[u8; 2]>::try_from(v.as_bytes()).unwrap();
        graph.entry(u).or_default().insert(v);
        graph.entry(v).or_default().insert(u);
    }
    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = build_graph(input);
    Some(
        graph
            .iter()
            .flat_map(|(u, vs)| {
                vs.iter().tuple_combinations().filter(|&(v, w)| {
                    [u[0], v[0], w[0]].contains(&b't') && graph.get(v).unwrap().contains(w)
                })
            })
            .count() as u32
            / 3,
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = build_graph(input);

    // The graph turns out regular; every node has the same number of nodes connected
    // dbg!(graph.values().map(|vs| vs.len()).unique().collect_vec());

    Some(
        graph
            .iter()
            .map(|(u, vs)| {
                vs.iter()
                    .powerset()
                    .collect_vec()
                    .into_iter()
                    .rev()
                    .filter(|ws| {
                        let ws_set: HashSet<[u8; 2]> =
                            HashSet::from_iter(ws.clone().into_iter().map(|&w| w));
                        ws.into_iter().all(|&&w| {
                            ws_set.is_subset(
                                &graph
                                    .get(&w)
                                    .unwrap()
                                    .union(&HashSet::from([w]))
                                    .map(|&w| w)
                                    .collect(),
                            )
                        })
                    })
                    .next()
                    .unwrap()
                    .into_iter()
                    .chain([u])
                    .collect_vec()
            })
            .max_by_key(|us| us.len())?
            .into_iter()
            .sorted_unstable()
            .flat_map(|v| String::from_utf8(v.to_vec()))
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
