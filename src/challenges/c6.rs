pub fn main() {
    // Create buckets based on the internal timer of each fish
    let mut buckets = [0; 9];

    // Add each fish in the input to its bucket
    include_str!("../../data/6.txt")
        .split(',')
        .for_each(|fish| {
            buckets[fish.parse::<usize>().unwrap()] += 1;
        });

    for day in 0..256 {
        // Gather all fishes with a timer of 0
        // They will be used to generate new fish and reset their own timers
        let new_fishes = buckets[0];
        // Decrease all fish timers by 1 by moving them to the next bucket
        for i in 0..8 {
            buckets[i] = buckets[i+1];
        }

        // Reset timers of all fishes to 6
        buckets[6] += new_fishes;
        // Create new fishes with a timer of 8
        buckets[8] = new_fishes;

        // Compute the sum for that date
        let total_fishes = buckets.iter().sum::<usize>();
        println!("Day {}: {} fishes", day, total_fishes);
    }
}