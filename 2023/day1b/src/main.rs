const NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn main() {
    println!(
        "{}",
        process(include_str!("../input.txt"))
    )
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            (0..line.len()).find_map(|i| num(line.as_bytes(), i)).unwrap() * 10 + 
            (0..line.len()).rev().find_map(|i| num(line.as_bytes(), i)).unwrap()
        })
        .sum::<usize>()
}

fn num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0').into())
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input));
        Ok(())
    }
}

