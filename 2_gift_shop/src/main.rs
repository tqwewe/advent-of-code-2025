use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let sum: u64 = INPUT
        .split(',')
        .map(|range| {
            let (min_str, max_str) = range.trim().split_once('-').unwrap();
            let min: u64 = min_str.parse().unwrap();
            let max: u64 = max_str.parse().unwrap();
            min..=max
        })
        .fold(HashSet::<u64>::new(), |mut repeated_pats, range| {
            for n in range {
                let s = n.to_string();

                for i in 1..(s.len() / 2) + 1 {
                    let rem = s.len() / i;
                    if rem != 2 {
                        continue;
                    }

                    let pat = &s[..i];
                    if pat.repeat(rem) == s {
                        repeated_pats.insert(s.parse().unwrap());
                    }
                }
            }

            repeated_pats
        })
        .into_iter()
        .sum::<u64>();

    println!("{sum}");
}
