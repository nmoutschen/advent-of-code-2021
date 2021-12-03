#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl From<&&str> for Command {
    fn from(s: &&str) -> Self {
        let args = s.split(' ').collect::<Vec<_>>();
        let amount = args[1].parse::<usize>().unwrap();

        match args[0] {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            s => panic!("Unknown command: {:?}", s),
        }
    }
}

#[derive(Debug)]
struct Position {
    pub x: usize,
    pub depth: usize,
    pub aim: usize,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn mov(&mut self, command: Command) {
        match command {
            Command::Forward(n) => {
                self.x += n;
                self.depth += self.aim * n;
            }
            Command::Down(n) => {
                self.aim += n;
            }
            Command::Up(n) => {
                self.aim -= n;
            }
        };
    }
}

pub fn main() {
    let data = include_str!("../../data/2.txt")
        .split('\n')
        .collect::<Vec<_>>();
    println!("Result: {}", challenge2(&data));
}

/// Takes a series of commands that moves the submarine and returns the product of the x and y coordinates
pub fn challenge2(input: &[&str]) -> usize {
    let mut pos = Position::new();
    for line in input {
        let command = Command::from(line);
        pos.mov(command);
    }

    pos.x * pos.depth
}
