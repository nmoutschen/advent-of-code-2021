use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct C5Error;

impl std::error::Error for C5Error {}

impl Display for C5Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C5Error")
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign<&Point> for Point
{
    fn add_assign(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(',') {
            Ok(Point {
                x: x.trim().parse::<i32>()?,
                y: y.trim().parse::<i32>()?,
            })
        } else {
            Err(Box::new(C5Error))
        }
    }
}

#[derive(Debug)]
struct Vent {
    from: Point,
    to: Point,
}

impl Vent {
    /// Vent line is vertical
    pub fn is_vert(&self) -> bool {
        self.from.x == self.to.x
    }

    /// Vent line is horizontal
    pub fn is_horiz(&self) -> bool {
        self.from.y == self.to.y
    }

    /// Vent line is diagonal
    pub fn is_diag(&self) -> bool {
        (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs()
    }

    pub fn add_to_map(&self, map: &mut Vec<i32>, width: i32) {
        // Only add to map if the line is horizontal, vertical, or a diagonal
        if !self.is_vert() && !self.is_horiz() && !self.is_diag() {
            return;
        }
        // Calculate the direction vector
        let direction = Point {
            x: (self.to.x - self.from.x).signum(),
            y: (self.to.y - self.from.y).signum(),
        };

        let mut pos = self.from.clone();

        while pos != self.to {
            map[(pos.y * width + pos.x) as usize] += 1;
            pos += &direction;
        }
        // Need one last addition for the final point
        map[(pos.y * width + pos.x) as usize] += 1;
    }
}

impl FromStr for Vent {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((from, to)) = s.split_once("->") {
            Ok(Vent {
                from: from.trim().parse::<Point>()?,
                to: to.trim().parse::<Point>()?,
            })
        } else {
            Err(Box::new(C5Error))
        }
    }
}

pub fn main() {
    let data = include_str!("../../data/5.txt")
        .split("\n")
        .collect::<Vec<_>>();

    let vents = data
        .iter()
        .map(|s| s.parse::<Vent>().unwrap())
        .collect::<Vec<_>>();

    // 10x10 size map
    let mut map = vec![0; 1000000];
    vents.iter().for_each(|v| v.add_to_map(&mut map, 1000));

    let mut points = 0;
    map.iter().for_each(|p| {
        if *p >= 2 {
            points += 1;
        }
    });

    let map_print = (0..=9).map(|y| {
        (0..=9).map(|x| {
            map[(y*10 + x) as usize]
        }).fold(String::new(), |acc, p| {
            acc + &format!(" {}", p)
        })
    }).fold(String::new(), |acc, p| {
        acc + &format!("\n{}", p)
    });
    println!("{}", map_print);

    println!("{}", points);
}