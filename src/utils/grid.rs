use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: isize,
    pub y: isize,
}

impl Add for GridCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for GridCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(isize, isize)> for GridCoord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Debug for GridCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Grid<T> where T: Default + Clone {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> where T: Default + Clone {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn in_bounds(&self, p: GridCoord) -> bool {
        (p.x as usize) < self.width && (p.y as usize) < self.height
    }

    /// cell_mut returns a mutable reference to the contents at the given location.
    pub fn cell_mut(&mut self, p: GridCoord) -> Option<&mut T> {
        match self.index_for_coord(p) {
            None => { None }
            Some(idx) => {
                Some(&mut self.data[idx])
            }
        }
    }

    /// cell returns a read-only reference to the contents at the given location.
    pub fn cell(&self, p: GridCoord) -> Option<&T> {
        match self.index_for_coord(p) {
            None => { None }
            Some(idx) => {
                Some(&self.data[idx])
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index_for_coord(&self, p: GridCoord) -> Option<usize> {
        if !self.in_bounds(p) {
            return None;
        }
        Some((p.y * (self.width as isize) + p.x) as usize)
    }

    pub fn grid_coordinates(&self) -> Vec<GridCoord> {
        let mut coords = vec![];
        for row in 0..self.height {
            for col in 0..self.width {
                coords.push((row as isize, col as isize).into())
            }
        }
        coords
    }
}
