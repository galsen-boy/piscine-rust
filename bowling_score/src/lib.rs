#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    current_frame: usize,
    is_extra_roll: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            current_frame: 1,
            is_extra_roll: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.current_frame > 10 && !self.is_extra_roll {
            return Err(Error::GameComplete);
        }

        if let Some(&last_roll) = self.rolls.last() {
            if last_roll + pins > 10 && (self.current_frame <= 10 || (self.current_frame == 10 && !self.is_extra_roll)) {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        self.rolls.push(pins);

        // Advance frame logic
        if self.current_frame <= 10 {
            if pins == 10 {
                // Strike
                if self.current_frame < 10 {
                    self.current_frame += 1;
                } else {
                    self.is_extra_roll = true;
                }
            } else if self.rolls.len() % 2 == 0 {
                // End of frame unless strike
                if self.current_frame < 10 || self.is_extra_roll {
                    self.current_frame += 1;
                    self.is_extra_roll = false;
                } else {
                    self.is_extra_roll = true;
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.current_frame <= 10 {
            return None;
        }

        let mut score = 0;
        let mut frame_index = 0;

        for _frame in 1..=10 {
            if frame_index >= self.rolls.len() {
                return None;
            }
            let first_roll = self.rolls[frame_index];
            if first_roll == 10 {
                // Strike
                if frame_index + 2 >= self.rolls.len() {
                    return None;
                }
                score += 10 + self.rolls[frame_index + 1] + self.rolls[frame_index + 2];
                frame_index += 1;
            } else {
                if frame_index + 1 >= self.rolls.len() {
                    return None;
                }
                let second_roll = self.rolls[frame_index + 1];
                if first_roll + second_roll == 10 {
                    // Spare
                    if frame_index + 2 >= self.rolls.len() {
                        return None;
                    }
                    score += 10 + self.rolls[frame_index + 2];
                } else {
                    score += first_roll + second_roll;
                }
                frame_index += 2;
            }
        }

        Some(score)
    }
}