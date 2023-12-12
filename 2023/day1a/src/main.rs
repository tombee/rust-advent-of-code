fn main() {
    println!(
        "{}",
        process(include_str!("../input.txt"))
    )
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars().find(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap() * 10 +
            line.chars().rev().find(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input));
        Ok(())
    }
}

