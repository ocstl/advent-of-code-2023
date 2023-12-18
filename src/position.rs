#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub y: isize,
    pub x: isize,
}

impl Position {
    pub const fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    pub const fn manhattan_distance(self, other: Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Position::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        self.x += rhs.dx;
        self.y += rhs.dy;
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Direction;

    fn sub(self, rhs: Position) -> Self::Output {
        Direction::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// DOWN and UP are reversed, for reasons...
pub const UP: Direction = Direction { dx: 0, dy: 1 };
pub const DOWN: Direction = Direction { dx: 0, dy: -1 };
pub const LEFT: Direction = Direction { dx: -1, dy: 0 };
pub const RIGHT: Direction = Direction { dx: 1, dy: 0 };
pub const UP_LEFT: Direction = Direction { dx: -1, dy: 1 };
pub const UP_RIGHT: Direction = Direction { dx: 1, dy: 1 };
pub const DOWN_LEFT: Direction = Direction { dx: -1, dy: -1 };
pub const DOWN_RIGHT: Direction = Direction { dx: 1, dy: -1 };
pub const DIRECTIONS: [Direction; 8] = [
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Direction {
    pub dx: isize,
    pub dy: isize,
}

impl Direction {
    pub const fn new(dx: isize, dy: isize) -> Self {
        Direction { dx, dy }
    }

    pub const fn signum(self) -> Self {
        Direction::new(self.dx.signum(), self.dy.signum())
    }

    pub fn maximum_norm(self) -> isize {
        self.dx.abs().max(self.dy.abs())
    }
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' => Ok(UP),
            'D' => Ok(DOWN),
            'L' => Ok(LEFT),
            'R' => Ok(RIGHT),
            _ => Err(value),
        }
    }
}
