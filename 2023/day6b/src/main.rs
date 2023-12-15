use std::collections::HashMap;

fn process(input: &str) -> usize {
    let parsed_input: HashMap<&str, usize> = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|e| {
                    (
                        e.0,
                        e.1.chars()
                            .filter(|c| !c.is_whitespace())
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    let time = parsed_input["Time"];
    let distance_record = parsed_input["Distance"];

    let mut record_options: usize = 0;
    for button_hold in 1..(time - 1) {
        let movement_time = time - button_hold;
        let distance = movement_time * button_hold;
        if distance > distance_record {
            record_options += 1;
        }
    }

    record_options
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

        assert_eq!(71503, process(input));
    }
}
