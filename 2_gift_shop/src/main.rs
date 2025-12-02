use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let ranges: Vec<_> = INPUT
        .split(',')
        .map(|range| {
            let (min_str, max_str) = range.trim().split_once('-').unwrap();
            let min: u64 = min_str.parse().unwrap();
            let max: u64 = max_str.parse().unwrap();
            min..=max
        })
        .collect();

    let mut repeated_pats: HashSet<u64> = HashSet::new();
    for range in ranges {
        // let mut repeated_pats: HashSet<u64> = HashSet::new();
        for n in range.clone() {
            let s = n.to_string();

            for i in 1..(s.len() / 2) + 1 {
                if !s.len().is_multiple_of(i) {
                    continue;
                }

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
    }

    let sum = repeated_pats.into_iter().sum::<u64>();
    println!("{sum}");
}
