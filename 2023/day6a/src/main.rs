use std::collections::HashMap;

fn process(input: &str) -> usize {
    let parsed_input: HashMap<&str, Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|e| {
                    (
                        e.0,
                        e.1.split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect();

    let races: Vec<(&usize, &usize)> = parsed_input["Time"]
        .iter()
        .zip(parsed_input["Distance"].iter())
        .collect();

    let mut record_options: Vec<usize> = vec![];
    for race in races.iter() {
        let mut race_record_options = 0;
        let time = *race.0;
        let distance_record = &(race.1);
        for button_hold in 1..(time - 1) {
            let movement_time = time - button_hold;
            let distance = movement_time * button_hold;
            if distance > **distance_record {
                race_record_options += 1;
            }
        }
        record_options.push(race_record_options);
    }

    record_options.iter().filter(|r| **r > 0).product()
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
        let input = "Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(288, process(input));
    }
}
