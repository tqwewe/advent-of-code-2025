const INPUT: &str = include_str!("../input.txt");

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let banks: Vec<&str> = INPUT.split('\n').collect();

    let mut total_joltage = 0;
    for bank in banks {
        let mut highest_joltage = 0;
        for i in 0..bank.len() {
            for j in (i + 1)..bank.len() {
                let joltage: u64 = format!("{}{}", &bank[i..i + 1], &bank[j..j + 1])
                    .parse()
                    .unwrap();
                highest_joltage = highest_joltage.max(joltage);
            }
        }

        total_joltage += highest_joltage;
    }

    println!("{total_joltage}");
}

fn part_2() {
    let banks: Vec<&str> = INPUT.trim().split('\n').collect();

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

    println!("{total_joltage}");
}

// Lol
// let mut highest_joltage = 0;
// for a in 0..bank.len() {
//     for b in (a + 1)..bank.len() {
//         for c in (b + 1)..bank.len() {
//             for d in (c + 1)..bank.len() {
//                 for e in (d + 1)..bank.len() {
//                     for f in (e + 1)..bank.len() {
//                         for g in (f + 1)..bank.len() {
//                             for h in (g + 1)..bank.len() {
//                                 for i in (h + 1)..bank.len() {
//                                     for j in (i + 1)..bank.len() {
//                                         for k in (j + 1)..bank.len() {
//                                             for l in (k + 1)..bank.len() {
//                                                 let joltage: u64 = format!(
//                                                     "{}{}{}{}{}{}{}{}{}{}{}{}",
//                                                     &bank[a..a + 1],
//                                                     &bank[b..b + 1],
//                                                     &bank[c..c + 1],
//                                                     &bank[d..d + 1],
//                                                     &bank[e..e + 1],
//                                                     &bank[f..f + 1],
//                                                     &bank[g..g + 1],
//                                                     &bank[h..h + 1],
//                                                     &bank[i..i + 1],
//                                                     &bank[j..j + 1],
//                                                     &bank[k..k + 1],
//                                                     &bank[l..l + 1]
//                                                 )
//                                                 .parse()
//                                                 .unwrap();
//                                                 highest_joltage =
//                                                     highest_joltage.max(joltage);
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
