#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        (self.pos.0 == other.pos.0)
            || (self.pos.1 == other.pos.1)
            || ((self.pos.0 - other.pos.0).abs() == (self.pos.1 - other.pos.1).abs())
    }
}
