#![allow(dead_code)]

use std::ops;

///
/// Coordinates
/// 

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coordinates(pub i64, pub i64);

impl ops::AddAssign<Coordinates> for Coordinates {
    fn add_assign(&mut self, rhs: Coordinates) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }

}

impl ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, _rhs: Coordinates) -> Coordinates {
        return Coordinates(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl ops::Mul<u64> for Coordinates {
    type Output = Coordinates;
    fn mul(self, rhs: u64) -> Self::Output {
        Coordinates(self.0 * rhs as i64, self.1 * rhs as i64)
    }
}

impl Coordinates {
    pub fn neighbors_cardinal(&self) -> [Coordinates; 4] {
        return [
            *self + NORTH,
            *self + EAST,
            *self + SOUTH,
            *self + WEST
        ]
    }
}

pub const NORTH_IDX: usize = 0;
pub const NORTH: Coordinates = Coordinates( 0, -1);
pub const SOUTH: Coordinates = Coordinates( 0,  1);
pub const EAST : Coordinates = Coordinates( 1,  0);
pub const WEST : Coordinates = Coordinates(-1,  0);

pub const NORTHEAST: Coordinates = Coordinates( 1, -1);
pub const SOUTHEAST: Coordinates = Coordinates( 1,  1);
pub const SOUTHWEST: Coordinates = Coordinates(-1,  1);
pub const NORTHWEST: Coordinates = Coordinates(-1, -1);

pub const CARDINALS: [Coordinates; 4] = [
    NORTH,
    EAST,
    SOUTH,
    WEST
];

pub const DIRS: [Coordinates; 8] = [
    NORTH,
    NORTHEAST,
    EAST,
    SOUTHEAST,
    SOUTH,
    SOUTHWEST,
    WEST,
    NORTHWEST
];

///
/// Grid
/// 

#[derive(Clone)]
pub struct Grid<T: Clone + Copy + PartialEq + Eq> {
    pub grid: Vec<Vec<T>>,
    pub max_size: Coordinates
}

impl<T: Clone + Copy + PartialEq + Eq> Grid<T> {
    pub fn from_string(str: &str, f: fn(char) -> T) -> Grid<T> {
        let grid: Vec<Vec<T>> = str
            .lines()
            .map(|l| l.chars().map(f).collect())
            .collect();

        let max_size = Coordinates(
            grid[0].len() as i64,
            grid.len() as i64
        );

        Grid {
            grid,
            max_size
        }
    }

    pub fn get_point(&self, coord: Coordinates) -> Option<T> {
        if !(0..self.max_size.0).contains(&coord.0) ||
            !(0..self.max_size.1).contains(&coord.1) {
            return None;
        }

        Some(self.grid[coord.1 as usize][coord.0 as usize])
    }

    pub fn set_point(&mut self, coord: Coordinates, v: T) {
        self.grid[coord.1 as usize][coord.0 as usize] = v;
    }

    pub fn find_one(&self, f: T) -> Option<Coordinates> {
        for (y, l) in self.grid.iter().enumerate() {
            for (x, v) in l.iter().enumerate() {
                if *v == f {
                    return Some(Coordinates(x as i64, y as i64));
                }
            }
        }

        None
    }

    pub fn get_max_size(&self) -> Coordinates {
        return self.max_size;
    }
}
