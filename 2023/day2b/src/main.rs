use atoi::atoi;

fn process(input: &[u8]) -> u32 {
    let mut result = 0;
    let lines = input.split(|b| b == &b'\n');
    for (i, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }

        let game_id = i + 1;
        let mut rgb = [0, 0, 0];

        let game_results = line
            .split(|b| b == &b':') // Split game ID from game results
            .nth(1) // Get the game results (everything after :)
            .unwrap_or_else(|| panic!("Couldn't find game results for game ID: {game_id}"));

        for game_result in game_results.split(|b| b == &b';' || b == &b',') {
            let game_result_split: Vec<_> = game_result.splitn(3, |b| b == &b' ').collect();

            let number_of_cubes = game_result_split[1];
            let colour_of_cubes = game_result_split[2];

            let colour_idx = match colour_of_cubes {
                b"red" => 0,
                b"green" => 1,
                b"blue" => 2,
                _ => unreachable!(),
            };

            rgb[colour_idx] =
                rgb[colour_idx].max(atoi(number_of_cubes).expect("couldn't parse number"))
        }

        result += rgb.into_iter().reduce(|a, b| a * b).unwrap();
    }

    result
}

fn main() {
    let input = include_bytes!("../input.txt");
    let output = process(input);
    println!("Answer: {}", output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .as_bytes();

        assert_eq!(2286, process(input));
    }
}
