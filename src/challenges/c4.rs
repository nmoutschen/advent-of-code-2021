#[derive(PartialEq, Debug, Clone)]
enum BingoCell {
    Number(i32),
    Selected,
}

impl From<i32> for BingoCell {
    fn from(n: i32) -> Self {
        BingoCell::Number(n)
    }
}

#[derive(Debug)]
pub struct Bingo {
    cells: Vec<BingoCell>,
}

impl Bingo {
    pub fn new(numbers: &[i32]) -> Self {
        let cells = numbers.iter().map(|&n| n.into()).collect();

        Bingo { cells }
    }

    pub fn set(&mut self, n: i32) {
        self.cells.iter_mut().for_each(|cell| {
            if let BingoCell::Number(number) = cell {
                if number == &n {
                    *cell = BingoCell::Selected;
                }
            }
        });
    }

    pub fn won(&self) -> bool {
        (0..5)
            .map(|i| {
                // Horizontal lines
                (i*5..i*5+5).all(|n| {
                    self.cells[n] == BingoCell::Selected
                }) ||
                // Vertical lines
                [i, i+5, i+10, i+15, i+20].iter().all(|n| {
                    self.cells[*n] == BingoCell::Selected
                })
            })
            .any(|b| b)
    }

    pub fn sum(&self) -> i32 {
        let mut sum = 0;
        for cell in &self.cells {
            if let BingoCell::Number(n) = cell {
                sum += n;
            }
        }
        sum
    }
}

impl std::str::FromStr for Bingo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i32> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
        Ok(Bingo::new(&numbers))
    }
}

impl std::fmt::Display for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = (0..5)
            .map(|line| {
                (0..5)
                    .map(|col| {
                        let cell = &self.cells[line * 5 + col];
                        if let BingoCell::Number(n) = cell {
                            format!("{:2}", n)
                        } else {
                            " *".to_string()
                        }
                    })
                    .fold(String::new(), |acc, s| acc + &s + " ")
            })
            .fold(String::new(), |acc, s| acc + &s + "\n");

        write!(f, "{}", string)
    }
}

pub fn main() {
    let data = include_str!("../../data/4.txt")
        .split("\n\n")
        .collect::<Vec<_>>();
    let numbers = data[0]
        .split(',')
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut bingo_boards = data[1..]
        .iter()
        .map(|s| s.parse::<Bingo>().unwrap())
        .collect::<Vec<_>>();

    // Iterate over all numbers
    for number in numbers {
        // Iterate over all bingo boards
        (0..bingo_boards.len())
            .map(|i| {
                // Set the number on the board if it's present
                bingo_boards[i].set(number);
                // If the board is won
                if bingo_boards[i].won() {
                    // And it's the last board
                    if bingo_boards.len() == 1 {
                        // Print the result
                        println!("{}", bingo_boards[0]);
                        println!("{}", bingo_boards[0].sum() * number);
                    }
                    // Return the board index for deletion
                    Some(i)
                } else {
                    // Skip - we'll keep the board
                    None
                }
            })
            // Remove all None values
            .flatten()
            // Need to collect to prevent borrowing twice as mutable
            .collect::<Vec<_>>()
            .iter()
            // Need to reverse the iterator to delete the last matching elements first
            .rev()
            // Delete all winning boards
            .for_each(|i| {
                bingo_boards.remove(*i);
            });

        
        // for bingo_board in &mut bingo_boards {
        //     bingo_board.set(number);
        //     if bingo_board.won() {
        //         println!("{}", bingo_board);
        //         println!("{}", bingo_board.sum() * number);
        //         return;
        //     }
        // }
    }
}
