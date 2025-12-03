use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part_1(input: &str) -> u32 {
    let banks: Vec<&str> = input.trim().split('\n').collect();

    let mut total_joltage = 0;
    for bank in banks {
        let mut highest_joltage = 0;
        for i in 0..bank.len() {
            for j in (i + 1)..bank.len() {
                let joltage: u32 = format!("{}{}", &bank[i..i + 1], &bank[j..j + 1])
                    .parse()
                    .unwrap();
                highest_joltage = highest_joltage.max(joltage);
            }
        }

        total_joltage += highest_joltage;
    }

    total_joltage
}

pub fn part_2(banks: &[&str]) -> u64 {
    banks
        .par_iter()
        .map(|bank| {
            let mut next_index = 0;
            let mut joltage = 0;
            for remaining in (1..13).rev() {
                let b = &bank[next_index..];
                let (index, next_highest) = b
                    .char_indices()
                    .take(b.len() - remaining + 1)
                    .max_by(|(a_idx, a_char), (b_idx, b_char)| {
                        a_char.cmp(b_char).then(a_idx.cmp(b_idx).reverse())
                    })
                    .unwrap();
                next_index += index + 1;
                joltage = joltage * 10 + next_highest.to_digit(10).unwrap() as u64;
            }

            joltage
        })
        .sum()
}

pub fn part_2_bank(bank: &str) -> u64 {
    let mut next_index = 0;
    let mut joltage = 0;
    for remaining in (1..13).rev() {
        let b = &bank[next_index..];
        let (index, next_highest) = unsafe {
            b.char_indices()
                .take(b.len() - remaining + 1)
                .max_by(|(a_idx, a_char), (b_idx, b_char)| {
                    a_char.cmp(b_char).then(a_idx.cmp(b_idx).reverse())
                })
                .unwrap_unchecked()
        };
        next_index += index + 1;
        joltage = joltage * 10 + next_highest.to_digit(10).unwrap() as u64;
    }

    joltage
}

pub fn part_1_kye(input: &str) -> u32 {
    let banks = parse_battery_banks(input);
    let mut bank_batteries: Vec<u32> = Vec::with_capacity(12);
    for bank in banks {
        let l = bank[..bank.len() - 1].iter().max().unwrap();
        let l_idx = bank.iter().position(|battery| battery == l).unwrap();
        let r = bank[(l_idx + 1)..bank.len()].iter().max().unwrap();
        bank_batteries.push((*l as u32 * 10) + *r as u32);
    }

    bank_batteries.iter().sum()
}

pub fn part_2_kye(banks: &[Vec<u32>]) -> u64 {
    banks.par_iter().map(|bank| part_2_kye_bank(bank)).sum()
}

#[inline]
pub fn part_2_kye_bank(mut bank: &[u32]) -> u64 {
    let mut joltage = 0;
    for batteries_len in 0..12 {
        if bank.is_empty() {
            break;
        }

        let needed = 12 - batteries_len;
        let max_idx = bank.len() - needed + 1;

        // SAFETY: bank[..max_idx] should have at least one item
        let (l_idx, l) = unsafe {
            bank[..max_idx]
                .iter()
                .enumerate()
                .max_by(|(a_idx, a), (b_idx, b)| a.cmp(b).then(b_idx.cmp(a_idx)))
                .unwrap_unchecked()
        };

        joltage = joltage * 10 + *l as u64;
        bank = &bank[(l_idx + 1)..];
    }

    joltage
}

pub fn parse_battery_banks(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .map(|bank| {
            bank.trim()
                .chars()
                .map(|battery| battery.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}
