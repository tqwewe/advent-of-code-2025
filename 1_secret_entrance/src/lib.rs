use std::str::FromStr;

use thiserror::Error;

pub const DIAL_INITIAL_VALUE: u8 = 50;
pub const DIAL_MAX_VALUE: u8 = 99;
pub const DIAL_COMBINATIONS: u8 = DIAL_MAX_VALUE + 1;

pub fn find_password_part_1(mut dial: u8, rotations: &[Rotation]) -> u32 {
    let mut zeros = 0;
    for rotation in rotations {
        dial = rotation.apply_to_dial(dial);
        if dial == 0 {
            zeros += 1;
        }
    }

    zeros
}

pub fn find_password_part_2(mut dial: u8, rotations: &mut [Rotation]) -> u32 {
    let mut zeros = 0;
    for rotation in rotations {
        while let (next_dial, true) = rotation.tick(dial) {
            dial = next_dial;
            if dial == 0 {
                zeros += 1;
            }
        }
    }

    zeros
}

/// A rotation instruction with a direction and amount.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rotation {
    pub direction: Direction,
    pub amount: u16,
}

impl Rotation {
    /// Parse a series of rotation instructions from a secret document.
    pub fn parse_from_document(document: &str) -> Result<Vec<Rotation>, ParseRotationError> {
        document
            .trim()
            .split('\n')
            .map(Rotation::from_str)
            .collect()
    }

    /// Applies a rotation fully to a dial, returning the new dial.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use secret_entrance::{ParseRotationError, Rotation};
    /// # fn main() -> Result<(), ParseRotationError> {
    /// assert_eq!(Rotation::from_str("R5")?.apply_to_dial(10), 15);
    /// assert_eq!(Rotation::from_str("R90")?.apply_to_dial(15), 5);
    /// assert_eq!(Rotation::from_str("R420")?.apply_to_dial(10), 30);
    /// assert_eq!(Rotation::from_str("L10")?.apply_to_dial(5), 95);
    /// assert_eq!(Rotation::from_str("L90")?.apply_to_dial(95), 5);
    /// assert_eq!(Rotation::from_str("L420")?.apply_to_dial(15), 95);
    /// assert_eq!(Rotation::from_str("L420")?.apply_to_dial(20), 0);
    /// assert_eq!(Rotation::from_str("R0")?.apply_to_dial(0), 0);
    /// assert_eq!(Rotation::from_str("R1")?.apply_to_dial(0), 1);
    /// assert_eq!(Rotation::from_str("L1")?.apply_to_dial(0), 99);
    /// assert_eq!(Rotation::from_str("L1")?.apply_to_dial(1), 0);
    /// assert_eq!(Rotation::from_str("R1")?.apply_to_dial(99), 0);
    /// assert_eq!(Rotation::from_str("L2")?.apply_to_dial(1), 99);
    /// assert_eq!(Rotation::from_str("R2")?.apply_to_dial(99), 1);
    /// assert_eq!(Rotation::from_str("L5")?.apply_to_dial(0), 95);
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply_to_dial(&self, dial: u8) -> u8 {
        let dial = dial as i32;
        let amount = match self.direction {
            Direction::Left => -(self.amount as i32),
            Direction::Right => self.amount as i32,
        };

        let next_dial = (dial + amount) % 100;
        match self.direction {
            Direction::Left if next_dial < 0 => (100 + next_dial) as u8,
            Direction::Left | Direction::Right => next_dial as u8,
        }
    }

    /// Ticks the dial by one value, returning the next dial value and whether there's more ticks to be made for this rotation.
    ///
    /// This was my last resort "cheat" approach, but what gave me the result in end for part 2.
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use secret_entrance::{ParseRotationError, Rotation};
    /// # fn main() -> Result<(), ParseRotationError> {
    /// let mut rotation = Rotation::from_str("R3")?;
    ///
    /// let (dial, more) = rotation.tick(50);
    /// assert_eq!(dial, 51);
    /// assert_eq!(more, true);
    ///
    /// let (dial, more) = rotation.tick(dial);
    /// assert_eq!(dial, 52);
    /// assert_eq!(more, true);
    ///
    /// let (dial, more) = rotation.tick(dial);
    /// assert_eq!(dial, 53);
    /// assert_eq!(more, false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn tick(&mut self, dial: u8) -> (u8, bool) {
        if self.amount == 0 {
            return (dial, false);
        }
        self.amount -= 1;

        let next_dial = match self.direction {
            Direction::Left => {
                if dial == 0 {
                    99
                } else {
                    dial - 1
                }
            }
            Direction::Right => {
                if dial == 99 {
                    0
                } else {
                    dial + 1
                }
            }
        };

        (next_dial, self.amount > 0)
    }
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, amount_str) = s
            .split_at_checked(1)
            .ok_or(ParseRotationError::InputTooShort)?;

        let direction = direction_str
            .parse()
            .map_err(|_| ParseRotationError::InvalidDirection)?;

        let amount = amount_str
            .parse()
            .map_err(|_| ParseRotationError::InvalidAmount)?;

        Ok(Rotation { direction, amount })
    }
}

#[derive(Debug, Error)]
pub enum ParseRotationError {
    #[error("input too short")]
    InputTooShort,
    #[error("invalid direction")]
    InvalidDirection,
    #[error("invalid amount")]
    InvalidAmount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = InvalidDirection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(InvalidDirection),
        }
    }
}

#[derive(Debug, Error)]
#[error("invalid direction")]
pub struct InvalidDirection;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const DOCUMENT: &str = include_str!("../example_input.txt");

        let rotations = DOCUMENT
            .trim()
            .split('\n')
            .map(Rotation::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(find_password_part_1(DIAL_INITIAL_VALUE, &rotations), 3);
    }

    #[test]
    fn part_2() {
        const DOCUMENT: &str = include_str!("../example_input.txt");

        let mut rotations = DOCUMENT
            .trim()
            .split('\n')
            .map(Rotation::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(find_password_part_2(DIAL_INITIAL_VALUE, &mut rotations), 6);
    }
}
