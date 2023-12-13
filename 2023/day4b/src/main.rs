use std::collections::HashSet;

fn process(input: &str) -> usize {
    let mut result = 0;
    let mut card_matches: Vec<usize> = Vec::new();
    let mut cards = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let (_, card) = line.split_once(':').expect("Expected to find card number");
        let (my_nums_card, win_nums_card) =
            card.split_once('|').expect("Expected to find win/my nums");
        let my_nums = my_nums_card.split_whitespace().collect::<HashSet<_>>();
        let win_nums = win_nums_card.split_whitespace().collect::<HashSet<_>>();
        let matches = my_nums.intersection(&win_nums).count();
        card_matches.push(matches);
        cards.push(i);
    }

    while let Some(card_num) = cards.pop() {
        (card_num + 1..=card_num + card_matches[card_num]).for_each(|i| {
            cards.push(i);
        });
        result += 1;
    }

    result
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, process(input));
    }
}
