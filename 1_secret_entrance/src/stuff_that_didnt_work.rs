impl Rotation {
    // My misterable attempt at part 2 which failed completely
    // /// ```
    // /// # use std::str::FromStr;
    // /// # use secret_entrance::Rotation;
    // /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    // /// assert_eq!(Rotation::from_str("R5")?.apply_to_dial(10), (15, 0));
    // /// assert_eq!(Rotation::from_str("R90")?.apply_to_dial(15), (5, 1));
    // /// assert_eq!(Rotation::from_str("R420")?.apply_to_dial(10), (30, 4));
    // /// assert_eq!(Rotation::from_str("L10")?.apply_to_dial(5), (95, 1));
    // /// assert_eq!(Rotation::from_str("L90")?.apply_to_dial(95), (5, 0));
    // /// assert_eq!(Rotation::from_str("L420")?.apply_to_dial(15), (95, 5));
    // /// assert_eq!(Rotation::from_str("L420")?.apply_to_dial(20), (0, 4));
    // /// assert_eq!(Rotation::from_str("R0")?.apply_to_dial(0), (0, 0));
    // /// assert_eq!(Rotation::from_str("R1")?.apply_to_dial(0), (1, 0));
    // /// assert_eq!(Rotation::from_str("L1")?.apply_to_dial(0), (99, 0));
    // /// assert_eq!(Rotation::from_str("L1")?.apply_to_dial(1), (0, 0));
    // /// assert_eq!(Rotation::from_str("R1")?.apply_to_dial(99), (0, 0));
    // /// assert_eq!(Rotation::from_str("L2")?.apply_to_dial(1), (99, 1));
    // /// assert_eq!(Rotation::from_str("R2")?.apply_to_dial(99), (1, 1));
    // /// assert_eq!(Rotation::from_str("L5")?.apply_to_dial(0), (95, 0));
    // /// # Ok(())
    // /// # }
    // /// ```
    // pub fn apply_to_dial(&self, dial: u8) -> (u8, u32) {
    //     let dial = dial as i32;
    //     assert!(dial <= 99, "dial is {dial}, but should be less than 100");
    //     let amount = match self.direction {
    //         Direction::Left => -(self.amount as i32),
    //         Direction::Right => self.amount as i32,
    //     };

    //     let mut zero_clicks = ((dial + amount) / 100).unsigned_abs();
    //     let next_dial = (dial + amount) % 100;
    //     dbg!(dial, amount, zero_clicks);
    //     let next_dial = match self.direction {
    //         Direction::Left if next_dial < 0 => {
    //             zero_clicks += 1;
    //             (100 + next_dial) as u8
    //         }
    //         Direction::Right => {
    //             if next_dial == 0 {
    //                 zero_clicks = zero_clicks.saturating_sub(1);
    //             }
    //             next_dial as u8
    //         }
    //         Direction::Left => next_dial as u8,
    //     };

    //     if dial == 0 {
    //         zero_clicks = zero_clicks.saturating_sub(1);
    //     }

    //     (next_dial, zero_clicks)
    // }

    /// Another failure attempt at part 2.
    /// Counts the number of full rotations.
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use secret_entrance::Rotation;
    /// assert_eq!(Rotation::from_str("R1000").unwrap().count_full_rotations(), 10);
    /// assert_eq!(Rotation::from_str("R100").unwrap().count_full_rotations(), 1);
    /// assert_eq!(Rotation::from_str("R99").unwrap().count_full_rotations(), 0);
    /// assert_eq!(Rotation::from_str("R10").unwrap().count_full_rotations(), 0);
    /// assert_eq!(Rotation::from_str("L1000").unwrap().count_full_rotations(), 10);
    /// assert_eq!(Rotation::from_str("L100").unwrap().count_full_rotations(), 1);
    /// assert_eq!(Rotation::from_str("L99").unwrap().count_full_rotations(), 0);
    /// assert_eq!(Rotation::from_str("L10").unwrap().count_full_rotations(), 0);
    /// ```
    pub fn count_full_rotations(&self) -> u8 {
        u8::try_from(self.amount as i32 / DIAL_COMBINATIONS as i32).unwrap()
    }
}
