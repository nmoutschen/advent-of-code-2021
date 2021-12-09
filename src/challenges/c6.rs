pub fn main() {
    let mut buckets = [0; 9];

    include_str!("../../data/6.txt")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|fish| {
            buckets[fish] += 1;
        });

    for day in 0..256 {
        let new_fishes = buckets[0];
        for i in 0..8 {
            buckets[i] = buckets[i+1];
        }

        buckets[6] += new_fishes;
        buckets[8] = new_fishes;

        let total_fishes = buckets.iter().sum::<usize>();
        println!("Day {}: {} fishes", day, total_fishes);
    }
}