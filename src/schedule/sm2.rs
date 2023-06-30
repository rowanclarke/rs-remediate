use std::{cmp::Ordering, io::stdin};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

use super::{Query, Review};

#[derive(Debug)]
pub struct Data {
    repeat: usize,
    difficulty: f32,
    interval: u32,
}

#[derive(IntoPrimitive, TryFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Score {
    Awful = 0,
    Poor = 1,
    Okay = 2,
    Good = 3,
    Solid = 4,
    Perfect = 5,
}

impl Score {
    fn score(&self) -> u8 {
        Into::<u8>::into(*self)
    }

    fn is_correct(&self) -> bool {
        self.score() > 2
    }
}

impl Query for Score {
    fn query() -> Self {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        Score::try_from(input.trim().parse::<u8>().unwrap()).unwrap()
    }
}

impl Review for Data {
    type Score = Score;

    fn review(&mut self, score: Score) {
        if score.is_correct() {
            self.interval = match self.repeat {
                0 => 1,
                1 => 6,
                _ => (self.interval as f32 * self.difficulty).round() as u32,
            };
            self.repeat += 1;
        } else {
            self.repeat = 0;
            self.interval = 1;
        }

        let diff = (5 - score.score()) as f32;
        self.difficulty += 0.1 - diff * (diff * 0.02 + 0.08);
        self.difficulty = f32::max(1.3, self.difficulty);
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            repeat: 0,
            difficulty: 0.5,
            interval: 0,
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl Eq for Data {}
