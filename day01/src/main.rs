use std::fs;

fn first(content: &str) -> u32 {
    let mut data = [Vec::new(), Vec::new()];
    content.lines().for_each(|line| {
        let vals = line.split_ascii_whitespace().collect::<Vec<_>>();
        data[0].push(vals.first().unwrap().parse::<u32>().unwrap());
        data[1].push(vals.get(1).unwrap().parse::<u32>().unwrap());
    });
    data[0].sort();
    data[1].sort();
    data[0]
        .iter()
        .zip(data[1].iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum()
}

fn second(content: &str) -> u32 {
    let mut data = [Vec::new(), Vec::new()];
    content.lines().for_each(|line| {
        let vals = line.split_ascii_whitespace().collect::<Vec<_>>();
        data[0].push(vals.first().unwrap().parse::<u32>().unwrap());
        data[1].push(vals.get(1).unwrap().parse::<u32>().unwrap());
    });
    data[0]
        .iter()
        .map(|val| val * data[1].iter().filter(|&v| val == v).count() as u32)
        .sum::<u32>()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(1151792, first(content));
    assert_eq!(21790168, second(content));
}
