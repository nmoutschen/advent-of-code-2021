pub fn main() {
    let data = include_str!("../../data/3.txt")
        .split('\n')
        .collect::<Vec<_>>();
    println!("Result: {}", challenge3_2(&data));
}

pub fn challenge3(input: &[&str]) -> usize {
    let line_size = input[0].len();
    let half_size = input.len() / 2;
    let mut counts = vec![0; line_size];

    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..counts.len() {
        if counts[i] > half_size {
            gamma += 1 << (counts.len() - i - 1);
        } else {
            epsilon += 1 << (counts.len() - i - 1);
        }
    }

    gamma * epsilon
}

pub fn count_match(input: &[usize], mask: usize) -> usize {
    input
        .iter()
        .map(|&x| (x & mask) == mask)
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}
pub fn get_most_common(input: &[usize], mask: usize) -> usize {
    let count = count_match(input, mask);

    let new_input: Vec<usize> = if count * 2 >= input.len() {
        input
            .iter()
            .filter(|&x| (x & mask) == mask)
            .copied()
            .collect()
    } else {
        input
            .iter()
            .filter(|&x| (x & mask) != mask)
            .copied()
            .collect()
    };

    if new_input.len() == 1 {
        new_input[0]
    } else {
        get_most_common(&new_input, mask >> 1)
    }
}

pub fn get_least_common(input: &[usize], mask: usize) -> usize {
    let count = count_match(input, mask);

    let new_input: Vec<usize> = if count * 2 < input.len() {
        input
            .iter()
            .filter(|&x| (x & mask) == mask)
            .copied()
            .collect()
    } else {
        input
            .iter()
            .filter(|&x| (x & mask) != mask)
            .copied()
            .collect()
    };

    if new_input.len() == 1 {
        new_input[0]
    } else {
        get_least_common(&new_input, mask >> 1)
    }
}

pub fn challenge3_2(input: &[&str]) -> usize {
    let line_size = input[0].len();

    let input = input
        .iter()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect::<Vec<_>>();

    let oxygen = get_most_common(&input, 1 << (line_size - 1));
    let co2_scrubber = get_least_common(&input, 1 << (line_size - 1));

    oxygen * co2_scrubber
}
