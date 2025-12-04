use printing_department::{part_1, part_2};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_solution = part_1(INPUT);
    println!("{part_1_solution}");

    let part_2_solution = part_2(INPUT);
    println!("{part_2_solution}");
}
