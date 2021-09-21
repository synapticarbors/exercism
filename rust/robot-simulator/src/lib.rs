// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    d: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::North => Robot {
                d: Direction::East,
                ..self
            },
            Direction::East => Robot {
                d: Direction::South,
                ..self
            },
            Direction::South => Robot {
                d: Direction::West,
                ..self
            },
            Direction::West => Robot {
                d: Direction::North,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::North => Robot {
                d: Direction::West,
                ..self
            },
            Direction::East => Robot {
                d: Direction::North,
                ..self
            },
            Direction::South => Robot {
                d: Direction::East,
                ..self
            },
            Direction::West => Robot {
                d: Direction::South,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, c| match c {
            'R' => r.turn_right(),
            'L' => r.turn_left(),
            'A' => r.advance(),
            _ => r,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
