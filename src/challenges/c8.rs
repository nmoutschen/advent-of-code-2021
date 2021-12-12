use itertools::Itertools;
use std::collections::HashMap;

/// Parse an input line
///
/// ## Mapping numbers
///
/// 0 - abcefg (6)
/// 1 - cf (2)
/// 2 - acdeg (5)
/// 3 - acdfg (5)
/// 4 - bcdf (4)
/// 5 - abdfg (5)
/// 6 - abdefg (6)
/// 7 - acf (3)
/// 8 - abcdefg (7)
/// 9 - abcdfg (6)
///
/// ## Segments
///
/// ```no_run
///  aaaa
/// b    c
/// b    c
///  dddd
/// e    f
/// e    f
///  gggg
/// ```
fn parse_line(line: &str) -> usize {
    let (patterns, output) = line.split_once('|').unwrap();

    // Mapping of all segments
    let mut segments: HashMap<char, char> = HashMap::new();

    // Count the number of segments
    let mut seg_chars = patterns.chars().collect::<Vec<_>>();
    seg_chars.sort();
    let seg_count = seg_chars.iter().dedup_with_count().collect::<Vec<_>>();

    // Num 1 & 4
    let num_1 = patterns
        .split_whitespace()
        .filter(|s| s.len() == 2)
        .collect::<Vec<_>>()[0];
    let num_4 = patterns
        .split_whitespace()
        .filter(|s| s.len() == 4)
        .collect::<Vec<_>>()[0];

    // Segment B
    seg_count
        .iter()
        .find(|(count, _)| *count == 6)
        .map(|(_, c)| {
            segments.insert(**c, 'b');
        });
    // Segment E
    seg_count
        .iter()
        .find(|(count, _)| *count == 4)
        .map(|(_, c)| {
            segments.insert(**c, 'e');
        });
    // Segment F
    seg_count
        .iter()
        .find(|(count, _)| *count == 9)
        .map(|(_, c)| {
            segments.insert(**c, 'f');
        });
    // Segments A & C
    seg_count
        .iter()
        .filter(|(count, _)| *count == 8)
        .for_each(|(_, c)| {
            // Num 1 is cf, so it contains c
            if num_1.contains(**c) {
                segments.insert(**c, 'c');
            } else {
                segments.insert(**c, 'a');
            }
        });
    // Segments D & G
    seg_count
        .iter()
        .filter(|(count, _)| *count == 7)
        .for_each(|(_, c)| {
            // Num 4 is bcdf, so it contains d
            if num_4.contains(**c) {
                segments.insert(**c, 'd');
            } else {
                segments.insert(**c, 'g');
            }
        });

    // // Sanity check
    // println!("{:?}", num_1);
    // println!("{:?}", num_4);
    // println!("{:?}", seg_count);
    // println!("{:?}", segments);
    // assert_eq!(segments.len(), 7);

    // Remap segments
    let output = output
        .chars()
        .map(|c| segments.get(&c).unwrap_or(&c).clone())
        .collect::<String>();
    output
        .split_whitespace()
        .map(|s| seg_to_num(s))
        .fold(0, |acc, n| acc * 10 + n)
}

fn seg_to_num(seg: &str) -> usize {
    let mut chars = seg.chars().collect::<Vec<_>>();
    chars.sort();
    let seg = chars.iter().collect::<String>();
    match seg.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("Unknown segment: {}", seg),
    }
}

pub fn main1() {
    let input = include_str!("../../data/8.txt");
    let total = input
        .split('\n')
        .map(|line| {
            let (_, output) = line.split_once('|').unwrap();
            output
                .trim()
                .split(' ')
                .map(|seg| {
                    // println!("{} {}", seg, seg.len());
                    if [2, 4, 3, 7].contains(&seg.len()) {
                        1
                    } else {
                        0
                    }
                })
                .fold(0, |acc, x| acc + x)
        })
        .fold(0, |acc, x| acc + x);
    println!("{}", total);
}

pub fn main() {
    let input = include_str!("../../data/8.txt");
    let res = input
        .split('\n')
        .map(|line| {
            let res = parse_line(line);
            println!("{}", res);
            res
        })
        .fold(0, |acc, x| acc + x);

    println!("Total: {}", res);
}
