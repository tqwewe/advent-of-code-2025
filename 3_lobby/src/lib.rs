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

pub fn part_2(input: &str) -> u64 {
    let banks: Vec<&str> = input.trim().split('\n').collect();

    let mut total_joltage = 0;
    for bank in banks {
        let mut next_index = 0;
        let mut s = String::with_capacity(12);
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
            s.push(next_highest);
        }

        total_joltage += s.parse::<u64>().unwrap();
    }

    total_joltage
}

pub fn part_1_kye(input: &str) -> u32 {
    let banks = parse_battery_banks(input);
    let mut bank_batteries: Vec<u32> = Vec::with_capacity(12);
    for bank in banks {
        let l = bank[..bank.len() - 1].iter().max().unwrap();
        let l_idx = bank.iter().position(|battery| battery == l).unwrap();
        let r = bank[(l_idx + 1)..bank.len()].iter().max().unwrap();
        bank_batteries.push((l * 10) + r);
    }

    bank_batteries.iter().sum()
}

pub fn part_2_kye(input: &str) -> u64 {
    let banks = parse_battery_banks(input);

    let mut bank_batteries = Vec::<u64>::default();
    for bank in banks {
        let mut remaining_batteries = bank;
        let mut batteries = Vec::with_capacity(12);
        for _ in 0..12 {
            if remaining_batteries.is_empty() {
                break;
            }

            let needed = 12 - batteries.len();
            let max_idx = remaining_batteries.len() - needed + 1;

            let l = remaining_batteries[..max_idx].iter().max().unwrap();
            let l_idx = remaining_batteries
                .iter()
                .position(|battery| battery == l)
                .unwrap();

            batteries.push(*l);
            remaining_batteries = remaining_batteries[(l_idx + 1)..].to_vec();
        }

        bank_batteries.push(
            batteries
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit as u64),
        );
    }

    bank_batteries.iter().sum()
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
