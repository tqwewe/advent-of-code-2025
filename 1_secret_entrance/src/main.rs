use secret_entrance::{
    DIAL_INITIAL_VALUE, ParseRotationError, Rotation, find_password_part_1, find_password_part_2,
};

const DOCUMENT: &str = include_str!("../input.txt");

fn main() -> Result<(), ParseRotationError> {
    let mut rotations = Rotation::parse_from_document(DOCUMENT)?;

    let password = find_password_part_1(DIAL_INITIAL_VALUE, &rotations);
    println!("Part 1 Password: {password}");

    let password = find_password_part_2(DIAL_INITIAL_VALUE, &mut rotations);
    println!("Part 2 Password: {password}");

    Ok(())
}
