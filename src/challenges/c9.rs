use itertools::Itertools;

struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

const FLOOR: u32 = 0;
const RIDGE: u32 = 9;

impl Matrix {
    fn get_low_points(&self) -> Vec<u32> {
        (0..self.width)
            .map(|x| {
                (0..self.height).filter_map(move |y| {
                    let val = self.data[x + y * self.width];
                    // Left cell
                    if x != 0 && val >= self.data[x - 1 + y * self.width] {
                        None
                    // Top cell
                    } else if y != 0 && val >= self.data[x + (y - 1) * self.width] {
                        None
                    // Right cell
                    } else if x != self.width - 1 && val >= self.data[x + 1 + y * self.width] {
                        None
                    // Bottom cell
                    } else if y != self.height - 1 && val >= self.data[x + (y + 1) * self.width] {
                        None
                    } else {
                        Some(val)
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Returns a matrix with only the highest points (value = 9)
    fn get_ridge_map(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            // Mapping values as 0 and 9. This is not great, since this uses
            // special values, but good enough for this challenge.
            data: self
                .data
                .iter()
                .map(|&val| if val == RIDGE { RIDGE } else { FLOOR })
                .collect(),
        }
    }

    fn find_basins(&self) -> Vec<usize> {
        let mut basin_map = self.get_ridge_map();

        let mut changed = 1;
        // Starts at RIDGE+1 to create basins
        let mut counter = RIDGE + 1;

        while changed != 0 {
            changed = 0;
            for x in 0..self.width {
                for y in 0..self.height {
                    let val = basin_map.data[x + y * self.width];
                    // If the cell is a ridge, we can continue
                    if val == RIDGE {
                        continue;
                    }

                    // Neighbouring cells
                    let left_val = if x != 0 {
                        basin_map.data[x - 1 + y * self.width]
                    } else {
                        RIDGE
                    };
                    let top_val = if y != 0 {
                        basin_map.data[x + (y - 1) * self.width]
                    } else {
                        RIDGE
                    };
                    let right_val = if x != self.width - 1 {
                        basin_map.data[x + 1 + y * self.width]
                    } else {
                        RIDGE
                    };
                    let bottom_val = if y != self.height - 1 {
                        basin_map.data[x + (y + 1) * self.width]
                    } else {
                        RIDGE
                    };

                    // If the cell touches a basin, set it to the basin number, starting with left and top
                    if left_val < val && ![FLOOR, RIDGE].contains(&left_val) {
                        basin_map.data[x + y * self.width] = left_val;
                        changed += 1;
                    } else if top_val < val && ![FLOOR, RIDGE].contains(&top_val) {
                        basin_map.data[x + y * self.width] = top_val;
                        changed += 1;
                    } else if right_val < val && ![FLOOR, RIDGE].contains(&right_val) {
                        basin_map.data[x + y * self.width] = right_val;
                        changed += 1;
                    } else if bottom_val < val && ![FLOOR, RIDGE].contains(&bottom_val) {
                        basin_map.data[x + y * self.width] = bottom_val;
                        changed += 1;
                    } else if val == FLOOR {
                        basin_map.data[x + y * self.width] = counter;
                        counter += 1;
                        changed += 1;
                    }
                }
            }
            // basin_map.data.chunks(basin_map.width).for_each(|line| {
            //     println!("{:?}", line);
            // });
            println!("Changed: {}", changed);
        }

        basin_map.data.chunks(basin_map.width).for_each(|line| {
            println!("{:?}", line);
        });

        // Get the size of all basins
        basin_map.data.sort();
        basin_map
            .data
            .iter()
            .dedup_with_count()
            .filter_map(|(size, num)| if *num == RIDGE { None } else { Some(size) })
            .collect()
    }
}

pub fn main() {
    let input = include_str!("../../data/9.txt");
    let data = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .flatten()
        .collect::<Vec<_>>();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let matrix = Matrix {
        width,
        height,
        data,
    };

    let mut basins = matrix.find_basins();
    basins.sort_by(|a, b| b.cmp(a));
    println!("Basins: {:?}", basins);

    let result = basins[0..3].iter().product::<usize>();
    println!("Result: {}", result);
}
