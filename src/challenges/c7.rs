pub fn main() {
    let crabs = include_str!("../../data/7.txt")
        .split(',')
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let min_crab = crabs.iter().min().unwrap().clone();
    let max_crab = crabs.iter().max().unwrap().clone();
    let mut min_cost = isize::MAX;
    let mut min_pos = 0;

    for pos in min_crab..=max_crab {
        let mut cost = 0;
        for crab in &crabs {
            // Triangle number formula
            let n = (crab - pos).abs();
            cost += n * (n + 1) / 2;
        }
        if cost < min_cost {
            min_cost = cost;
            min_pos = pos;
        }
    }

    println!("POS: {}, COST: {}", min_pos, min_cost);
}
