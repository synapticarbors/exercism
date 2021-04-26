use std::iter;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Copy, Clone)]
enum Frame {
    NoRoll,
    Partial(u16),
    Open(u16, u16),
    Spare(u16, u16),
    Strike,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: vec![] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_finished() {
            return Err(Error::GameComplete);
        }

        if let Some(frame) = self.frames.last_mut() {
            match frame {
                Frame::Open(_, _) | Frame::Strike | Frame::Spare(_, _) => match pins {
                    p if p > 10 => return Err(Error::NotEnoughPinsLeft),
                    p if p == 10 => self.frames.push(Frame::Strike),
                    _ => self.frames.push(Frame::Partial(pins)),
                },
                Frame::Partial(x) => match x {
                    x if *x + pins > 10 => return Err(Error::NotEnoughPinsLeft),
                    x if *x + pins == 10 => *frame = Frame::Spare(*x, pins),
                    _ => *frame = Frame::Open(*x, pins),
                },
                _ => (),
            }
        } else {
            match pins {
                p if p > 10 => return Err(Error::NotEnoughPinsLeft),
                p if p == 10 => self.frames.push(Frame::Strike),
                _ => self.frames.push(Frame::Partial(pins)),
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_finished() {
            return None;
        }

        let score = self
            .frames
            .iter()
            .chain(iter::repeat(&Frame::NoRoll))
            .take(12)
            .collect::<Vec<_>>()
            .windows(3)
            .map(|w| match (w[0], w[1], w[2]) {
                (Frame::Open(a, b), _, _) => a + b,
                (Frame::Spare(a, b), Frame::Open(c, _), _) => a + b + c,
                (Frame::Spare(_, _), Frame::Spare(c, _), _) => 10 + c,
                (Frame::Spare(_, _), Frame::Strike, _) => 20,
                (Frame::Spare(_, _), Frame::Partial(a), _) => 10 + a,
                (Frame::Strike, Frame::Strike, Frame::Strike) => 30,
                (Frame::Strike, Frame::Strike, Frame::Open(a, _)) => 20 + a,
                (Frame::Strike, Frame::Strike, Frame::Spare(a, _)) => 20 + a,
                (Frame::Strike, Frame::Spare(_, _), _) => 20,
                (Frame::Strike, Frame::Open(a, b), _) => 10 + a + b,
                _ => 0,
            })
            .sum();

        Some(score)
    }

    fn is_finished(&self) -> bool {
        matches!(
            (self.frames.get(9), self.frames.get(10), self.frames.get(11)),
            (Some(Frame::Open(_, _)), None, None)
                | (
                    Some(Frame::Strike),
                    Some(Frame::Strike),
                    Some(Frame::Strike),
                )
                | (Some(Frame::Strike), Some(Frame::Open(_, _)), None)
                | (Some(Frame::Strike), Some(Frame::Spare(_, _)), None)
                | (Some(Frame::Spare(_, _)), Some(Frame::Partial(_)), None)
                | (Some(Frame::Spare(_, _)), Some(Frame::Strike), None)
        )
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
