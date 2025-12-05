use std::ops::RangeInclusive;

pub fn part_1(input: &str) -> u64 {
    let (start, end) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<RangeInclusive<u64>> = start
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            let from = from.parse().unwrap();
            let to = to.parse().unwrap();
            from..=to
        })
        .collect();

    let ingredient_ids: Vec<u64> = end.lines().map(|line| line.parse().unwrap()).collect();

    let mut fresh_count = 0;
    for ingredient_id in ingredient_ids {
        let is_fresh = fresh_ranges
            .iter()
            .any(|range| range.contains(&ingredient_id));
        if is_fresh {
            fresh_count += 1;
        }
    }

    fresh_count
}

pub fn part_2(input: &str) -> u64 {
    let (start, _end) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = start
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            let from = from.parse().unwrap();
            let to = to.parse().unwrap();
            from..=to
        })
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(b.end().cmp(a.end())));

    let mut last_end = 0;
    let mut sum = 0;
    for range in ranges {
        if *range.end() <= last_end {
            continue; // It overlaps entirely
        }

        let new_range = if *range.start() <= last_end {
            (last_end + 1)..=*range.end() // It overlaps slightly, bump the start
        } else {
            range
        };

        last_end = *new_range.end();
        sum += new_range.end() - new_range.start() + 1;
    }

    sum
}

pub fn part_2_kye(input: &str) -> u64 {
    let (start, _end) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<RangeInclusive<u64>> = start
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            let from = from.parse().unwrap();
            let to = to.parse().unwrap();
            from..=to
        })
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(b.end().cmp(a.end())));

    let mut valid_ids = 0;
    let mut s = *ranges[0].start();
    let mut e = *ranges[0].end();
    for r in ranges.iter().skip(1).take(ranges.len() - 2) {
        if *r.start() > e + 1 {
            valid_ids += (e - s) + 1;
            s = *r.start();
            e = *r.end();
        } else {
            e = e.max(*r.end());
        }
    }
    let last = ranges.last().unwrap();
    if *last.start() > e + 1 {
        valid_ids += (e - s) + 1;
        valid_ids += (last.end() - last.start()) + 1;
    } else {
        valid_ids += (e.max(*last.end()) - s) + 1;
    }
    valid_ids
}
