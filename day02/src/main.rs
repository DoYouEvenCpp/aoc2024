use is_sorted::IsSorted;
use itertools::Itertools;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum ReportStatus {
    Safe,
    Unsafe,
    Dampered(Vec<u32>), //YAGNI
}

fn is_safe(line: &[u32]) -> bool {
    (line.iter().is_sorted() || line.iter().is_sorted_by(|a, b| Some(b.cmp(a))))
        && line
            .iter()
            .tuple_windows()
            .all(|(lhs, rhs)| lhs.abs_diff(*rhs) > 0 && lhs.abs_diff(*rhs) <= 3)
}

fn safe_by_damper(line: &Vec<u32>) -> ReportStatus {
    if is_safe(line) {
        return ReportStatus::Safe;
    }
    for i in 0..line.len() {
        let mut copy = line.to_vec();
        copy.remove(i);
        if is_safe(&copy) {
            return ReportStatus::Dampered(copy);
        }
    }
    ReportStatus::Unsafe
}

fn first(content: &str) -> u32 {
    content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| match safe_by_damper(line) {
            ReportStatus::Unsafe => false,
            _ => true,
        })
        .count() as u32
}
fn second(content: &str) -> u32 {
    content
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| match safe_by_damper(line) {
            ReportStatus::Unsafe => false,
            _ => true,
        })
        .count() as u32
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(314, first(content));
    assert_eq!(373, second(content));
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn test_damper() {
        assert_eq!(safe_by_damper(&vec![1, 2, 3, 4, 5]), ReportStatus::Safe);
        assert_eq!(
            safe_by_damper(&vec![1, 3, 2, 4, 5]),
            ReportStatus::Dampered(vec![1, 2, 4, 5])
        );
        assert_eq!(
            safe_by_damper(&vec![8, 6, 4, 4, 1]),
            ReportStatus::Dampered(vec![8, 6, 4, 1])
        );
        assert_eq!(safe_by_damper(&vec![1, 2, 7, 8, 9]), ReportStatus::Unsafe);
        assert_eq!(safe_by_damper(&vec![9, 7, 6, 2, 1]), ReportStatus::Unsafe);
    }
}
