fn parse_int(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

pub fn main() {
    let data = include_str!("../../data/1.txt")
        .split('\n')
        .collect::<Vec<_>>();
    println!("Count: {}", challenge1(3, &data));
}

pub fn challenge1(window: usize, input: &[&str]) -> usize {
    let mut count = 0;

    let values = input.iter().map(|s| parse_int(s)).collect::<Vec<_>>();

    for i in window..values.len() {
        let prev_sum = values[i - window..i].iter().sum::<usize>();
        let curr_sum = values[i - window + 1..=i].iter().sum::<usize>();

        if prev_sum < curr_sum {
            count += 1;
        }
    }

    count
}
