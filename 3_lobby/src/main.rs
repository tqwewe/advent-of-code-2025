use lobby::{parse_battery_banks, part_1, part_2, part_2_kye};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let banks: Vec<&str> = INPUT.trim().split('\n').collect();
    println!("{}", part_1(INPUT));
    println!("{}", part_2(&banks));
    println!("{}", part_2_kye(&parse_battery_banks(INPUT)));
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
