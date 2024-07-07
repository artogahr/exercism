#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    rolls: Vec<u16>,
}

impl Frame {
    fn is_strike(&self) -> bool {
        self.rolls.get(0) == Some(&10)
    }

    fn is_spare(&self) -> bool {
        self.rolls.len() == 2 && self.rolls.iter().sum::<u16>() == 10
    }

    fn score(&self) -> u16 {
        self.rolls.iter().sum()
    }
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::new(),
            current_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.current_frame == 9 {
            self.handle_tenth_frame(pins)?;
        } else {
            self.handle_regular_frame(pins)?;
        }

        Ok(())
    }

    fn handle_regular_frame(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() <= self.current_frame {
            self.frames.push(Frame { rolls: vec![pins] });
            if pins == 10 {
                self.current_frame += 1;
            }
        } else {
            let frame = &mut self.frames[self.current_frame];
            if frame.rolls.len() == 1 && frame.rolls[0] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            frame.rolls.push(pins);
            self.current_frame += 1;
        }
        Ok(())
    }

    fn handle_tenth_frame(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() < 10 {
            self.frames.push(Frame { rolls: Vec::new() });
        }

        let frame = &mut self.frames[9];

        match frame.rolls.len() {
            0 => {
                frame.rolls.push(pins);
            }
            1 => {
                if frame.rolls[0] == 10 {
                    // First roll was a strike
                    frame.rolls.push(pins);
                    return Ok(());
                }
                if frame.rolls[0] + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                frame.rolls.push(pins);
            }
            2 => {
                if frame.rolls[0] == 10 {
                    // After a strike, the next two rolls can't exceed 10 pins total
                    if frame.rolls[1] + pins > 10 && frame.rolls[1] != 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    frame.rolls.push(pins);
                } else if frame.rolls[0] + frame.rolls[1] == 10 {
                    // After a spare, allow any valid roll
                    frame.rolls.push(pins);
                } else {
                    return Err(Error::GameComplete);
                }
            }
            _ => return Err(Error::GameComplete),
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_complete() {
            return None;
        }

        let mut total_score = 0;
        for i in 0..10 {
            let frame = &self.frames[i];
            total_score += frame.score();

            if i < 9 {
                if frame.is_strike() {
                    total_score += self.strike_bonus(i);
                } else if frame.is_spare() {
                    total_score += self.spare_bonus(i);
                }
            }
        }

        Some(total_score)
    }

    fn strike_bonus(&self, frame_index: usize) -> u16 {
        let next_frame = &self.frames[frame_index + 1];
        if next_frame.is_strike() && frame_index < 8 {
            10 + self.frames[frame_index + 2].rolls[0]
        } else {
            next_frame.rolls.iter().take(2).sum()
        }
    }

    fn spare_bonus(&self, frame_index: usize) -> u16 {
        self.frames[frame_index + 1].rolls[0]
    }

    fn is_game_complete(&self) -> bool {
        self.current_frame == 9 && self.frames.len() == 10 && {
            let tenth_frame = &self.frames[9];
            tenth_frame.rolls.len() == 2 && !tenth_frame.is_strike() && !tenth_frame.is_spare()
                || tenth_frame.rolls.len() == 3
        }
    }
}
