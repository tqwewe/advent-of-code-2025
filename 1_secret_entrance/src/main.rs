use std::error::Error;
use std::str::FromStr;

use secret_entrance::{DIAL_INITIAL_VALUE, Rotation, find_password_part_1, find_password_part_2};

const DOCUMENT: &str = include_str!("../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let mut rotations = DOCUMENT
        .trim()
        .split('\n')
        .map(Rotation::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let password = find_password_part_1(DIAL_INITIAL_VALUE, &rotations);
    println!("Part 1 Password: {password}");
    let password = find_password_part_2(DIAL_INITIAL_VALUE, &mut rotations);
    println!("Part 2 Password: {}", password);

    Ok(())
}
