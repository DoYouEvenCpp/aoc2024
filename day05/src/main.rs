use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn parse_input(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::<u32, Vec<u32>>::new();
    input.lines().for_each(|l| {
        if l.contains('|') {
            let v = l.split('|').collect::<Vec<_>>();
            map.entry(v[0].parse::<u32>().unwrap())
                .or_default()
                .push(v[1].parse::<u32>().unwrap());
        }
    });
    map
}

fn part_1(input: &str, map: &HashMap<u32, Vec<u32>>) -> u32 {
    input
        .lines()
        .filter(|l| !l.contains('|'))
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| {
            for (a, b) in v.iter().tuple_windows() {
                if !map.get(b).unwrap_or(&vec![]).contains(a) {
                    return false;
                }
            }
            true
        })
        .map(|vals| vals[vals.len() / 2])
        .sum()
}

fn part_2(input: &str, map: &HashMap<u32, Vec<u32>>) -> u32 {
    input
        .lines()
        .filter(|l| !l.contains('|'))
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| {
            for (a, b) in v.iter().tuple_windows() {
                if !map.get(b).unwrap_or(&vec![]).contains(a) {
                    return true;
                }
            }
            false
        })
        .map(|vals| {
            let mut fixed_order_map: HashMap<u32, usize> = HashMap::new();
            for order in vals.iter() {
                let count = vals
                    .iter()
                    .filter(|o| *o != order)
                    .filter(|o| map.get(order).unwrap_or(&vec![]).contains(o))
                    .count();
                fixed_order_map.insert(*order, count);
            }
            let sorted: Vec<u32> = fixed_order_map
                .into_iter()
                .sorted_by(|a, b| b.1.cmp(&a.1))
                .map(|(key, _v)| key)
                .collect::<Vec<_>>();
            sorted[sorted.len() / 2]
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input").expect("File not found!");
    let input = input.trim();
    let map = parse_input(input);

    assert_eq!(4905, part_1(input, &map));
    assert_eq!(6204, part_2(input, &map));
}
