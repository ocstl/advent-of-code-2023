use std::cmp::Ordering;
use std::iter::{Skip, StepBy, Take};
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    grid: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(height: usize, width: usize, grid: Vec<T>) -> Option<Self> {
        if height * width == grid.len() {
            Some(Grid {
                height,
                width,
                grid,
            })
        } else {
            None
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        if position.x < self.width {
            self.grid.get(position.x + position.y * self.width)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        if position.x < self.width {
            self.grid.get_mut(position.x + position.y * self.width)
        } else {
            None
        }
    }

    pub fn get_mut_tuple(
        &mut self,
        positions: (Position, Position),
    ) -> Option<(Option<&mut T>, Option<&mut T>)> {
        if positions.0 == positions.1 {
            return None;
        }

        let p1 = positions.0.x + positions.0.y * self.width;
        let p2 = positions.1.x + positions.1.y * self.width;

        match p1.cmp(&p2) {
            Ordering::Equal => None,
            Ordering::Greater => {
                if p1 <= self.grid.len() {
                    let (left, right) = self.grid.split_at_mut(p1);
                    Some((right.first_mut(), left.get_mut(p2)))
                } else {
                    Some((None, self.grid.get_mut(p2)))
                }
            }
            Ordering::Less => {
                if p2 <= self.grid.len() {
                    let (left, right) = self.grid.split_at_mut(p2);
                    Some((left.get_mut(p1), right.first_mut()))
                } else {
                    Some((self.grid.get_mut(p1), None))
                }
            }
        }
    }

    pub fn row(&self, idy: usize) -> Take<Skip<Iter<'_, T>>> {
        self.grid.iter().skip(idy * self.width).take(self.width)
    }

    pub fn row_mut(&mut self, idy: usize) -> Take<Skip<IterMut<'_, T>>> {
        self.grid.iter_mut().skip(idy * self.width).take(self.width)
    }

    pub fn column(&self, idx: usize) -> Take<StepBy<Skip<Iter<'_, T>>>> {
        self.grid
            .iter()
            .skip(idx)
            .step_by(self.width)
            .take(self.height)
    }

    pub fn column_mut(&mut self, idx: usize) -> Take<StepBy<Skip<IterMut<'_, T>>>> {
        self.grid
            .iter_mut()
            .skip(idx)
            .step_by(self.width)
            .take(self.height)
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.height).map(|idy| self.row(idy))
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.width).map(|idx| self.column(idx))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn x(self) -> usize {
        self.x
    }

    pub fn y(self) -> usize {
        self.y
    }

    pub const fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        self.x
            .checked_sub(1)
            .map(|x| Position::new(x, self.y))
            .into_iter()
            .chain(Some(Position::new(self.x + 1, self.y)))
            .chain(self.y.checked_sub(1).map(|y| Position::new(self.x, y)))
            .chain(Some(Position::new(self.x, self.y + 1)))
    }

    pub fn extended_neighbours(self) -> impl Iterator<Item = Self> {
        let min_x = self.x.saturating_sub(1);
        let max_x = self.x + 1;
        let min_y = self.y.saturating_sub(1);
        let max_y = self.y + 1;

        (min_x..=max_x)
            .flat_map(move |idx| (min_y..=max_y).map(move |idy| Position::new(idx, idy)))
            .filter(move |p| p.x != self.x || p.y != self.y)
    }

    pub fn manhattan_distance(self, other: Self) -> usize {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        dx + dy
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Option<Self>;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => self.y.checked_sub(1).map(|y| Position::new(self.x, y)),
            Direction::Down => Some(Position::new(self.x, self.y + 1)),
            Direction::Left => self.x.checked_sub(1).map(|x| Position::new(x, self.y)),
            Direction::Right => Some(Position::new(self.x + 1, self.y)),
            Direction::UpLeft => self
                .x
                .checked_sub(1)
                .and_then(|x| self.y.checked_sub(1).map(|y| Position::new(x, y))),
            Direction::DownLeft => self.x.checked_sub(1).map(|x| Position::new(x, self.y + 1)),
            Direction::UpRight => self.y.checked_sub(1).map(|y| Position::new(self.x + 1, y)),
            Direction::DownRight => Some(Position::new(self.x + 1, self.y + 1)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

impl Direction {
    pub fn rotate_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::UpLeft => Direction::DownLeft,
            Direction::DownLeft => Direction::DownRight,
            Direction::UpRight => Direction::UpLeft,
            Direction::DownRight => Direction::UpRight,
        }
    }

    pub fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::UpLeft => Direction::UpRight,
            Direction::DownLeft => Direction::UpLeft,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::UpLeft => Direction::DownRight,
            Direction::DownLeft => Direction::UpRight,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownRight => Direction::UpLeft,
        }
    }
}
