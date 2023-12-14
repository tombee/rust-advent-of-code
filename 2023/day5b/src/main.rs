use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    src_range: Range<i64>,
    delta: i64,
}

fn process(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let seeds: &str = lines[0].split_once(": ").unwrap().1;
    let seeds: Vec<i64> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps: Vec<Vec<Mapping>> = vec![];
    let mut map: Vec<Mapping> = vec![];
    for line in lines[2..].iter() {
        if line.contains(" map:") {
            continue;
        }
        if line.trim().is_empty() {
            if !map.is_empty() {
                maps.push(map);
                map = vec![];
            }
            continue;
        }

        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let src = nums[1];
        let dst = nums[0];
        let len = nums[2];

        map.push(Mapping {
            src_range: Range {
                start: src,
                end: src + len,
            },
            delta: dst - src,
        });
    }
    if !map.is_empty() {
        maps.push(map);
    }

    let mut min_location = i64::MAX;
    for seed_range in seeds.chunks(2) {
        for seed in seed_range[0]..seed_range[0] + seed_range[1] {
            let mut idx = seed;
            for map in maps.iter() {
                for mapping in map {
                    if mapping.src_range.contains(&idx) {
                        idx += mapping.delta;
                        break;
                    }
                }
            }
            if min_location > idx {
                min_location = idx;
            }
        }
    }

    min_location
}

fn main() {
    let input = include_str!("../input.txt");
    let output = process(input);
    println!("Answer: {}", output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, process(input));
    }
}
