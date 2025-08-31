use ilog::IntLog;
use std::collections::HashMap;

fn my_10_pow(val: u64) -> u64 {
    if val == 0 {
        return 1;
    }
    let mut res = 1u64;
    for _ in 0..val {
        res = res * 10;
    }
    res
}

fn get_digits(val: u64) -> u64 {
    (val.log10() + 1) as u64
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (key, val) in stones {
        match key {
            0 => *new_stones.entry(1).or_default() += val,
            _ if (0 == (get_digits(key) % 2)) => {
                let digits = get_digits(key);
                let pow = my_10_pow(digits / 2);
                let first = ((key as f64) / pow as f64) as u64;
                let second = key % pow;
                *new_stones.entry(first).or_default() += val;
                *new_stones.entry(second).or_default() += val;
            }
            _ => {
                *new_stones.entry(key * 2024).or_default() += val;
            }
        }
    }
    new_stones
}

fn part_1(input: &HashMap<u64, u64>, blinks: u64) -> usize {
    let mut stones = HashMap::<u64, u64>::with_capacity(input.len());
    stones = input.clone();
    for _ in 0..blinks {
        stones = blink(stones.clone());
    }
    //     stones.iter().for_each(|val| match val {
    //         0 => new_stones.push(1),
    //         _ if (0 == (get_digits(*val) % 2)) => {
    //             let digits = get_digits(*val);
    //             let pow = my_10_pow(digits / 2);
    //             let first = ((*val as f64) / pow as f64) as u64;
    //             let second = val % pow;
    //             new_stones.push(first);
    //             new_stones.push(second);
    //         }
    //         _ => {
    //             new_stones.push(val * 2024);
    //         }
    //     });
    //     stones = new_stones.to_vec();
    //     new_stones.clear();
    // }
    // stones.len()
    stones.len()
}

fn main() {
    // let input: Vec<u64> = vec![8793800, 1629, 65, 5, 960, 0, 138983, 85629];
    let mut input = HashMap::<u64, u64>::new();
    for val in vec![8793800u64, 1629, 65, 5, 960, 0, 138983, 85629].into_iter() {
        *input.entry(val).or_default() += 1;
    }
    // println!("First puzzle: {}", part_1(input.to_vec(), 25));
    println!("Second puzzle: {}", part_1(&input, 75));
    assert_eq!(231532558973909, part_1(&input, 75));
}

#[cfg(test)]
mod day11 {
    use super::*;
    fn testable(val: u64) -> u64 {
        (val.log10() + 1) as u64
    }
    fn testable_split(val: u64) -> (u64, u64) {
        let digits = testable(val);
        let pow = my_10_pow(digits / 2);
        (val / pow, val % pow)
    }
    #[test]
    fn digits_count() {
        assert_eq!(1, testable(5));
        assert_eq!(2, testable(10));
        assert_eq!(2, testable(25));
        assert_eq!(3, testable(243));
        assert_eq!(3, testable(999));
        assert_eq!(4, testable(1001));
        assert_eq!(5, testable(35390));
        assert_eq!(6, testable(253000));
    }

    #[test]
    fn my_pow_test() {
        assert_eq!(1, my_10_pow(0));
        assert_eq!(10, my_10_pow(1));
        assert_eq!(100, my_10_pow(2));
        assert_eq!(1000, my_10_pow(3));
        assert_eq!(100000, my_10_pow(5));
    }
    #[test]
    fn split_number() {
        assert_eq!((1, 7), testable_split(17));
        assert_eq!((253, 0), testable_split(253000));
    }
}
