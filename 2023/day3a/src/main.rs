use std::collections::HashSet;

struct PartNum {
    number: usize,
    adjacents: HashSet<(isize, isize)>,
}

impl PartNum {
    fn new(num: usize, row: isize, col: isize) -> Self {
        Self {
            number: num,
            adjacents: HashSet::from([
                (row - 1, col - 1), // Upper Left
                (row, col - 1),     // Left
                (row + 1, col - 1), // Lower Left
                (row - 1, col),     // Above
                (row + 1, col),     // Below
                (row - 1, col + 1), // Upper Right
                (row, col + 1),     // Right
                (row + 1, col + 1), // Lower Right
            ]),
        }
    }
    fn append_number(&mut self, num: usize, row: isize, col: isize) {
        self.number = self.number * 10 + num;
        self.adjacents.extend([
            (row - 1, col + 1), // Upper Right
            (row, col + 1),     // Right
            (row + 1, col + 1), // Lower Right
        ]);
        self.adjacents.remove(&(row, col));
    }
}

fn process(input: &str) -> usize {
    let mut part_numbers: Vec<PartNum> = Vec::new();
    let mut symbols: HashSet<(isize, isize)> = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        let mut part_num: Option<PartNum> = None;
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                let num = (ch as u8 - b'0') as usize;
                if let Some(ref mut pnum) = part_num {
                    pnum.append_number(num, row as isize, col as isize);
                } else {
                    part_num = Some(PartNum::new(num, row as isize, col as isize));
                }
            } else {
                if ch != '.' {
                    symbols.insert((row as isize, col as isize));
                }
                if let Some(pnum) = part_num.take() {
                    part_numbers.push(pnum);
                }
            }
        }
        // If we reach the end of the line, check if we have a part number being
        // built and push it before moving to the next line
        if let Some(pnum) = part_num.take() {
            part_numbers.push(pnum);
        }
    }

    part_numbers
        .iter()
        .filter(|p| !symbols.is_disjoint(&p.adjacents))
        .map(|p| p.number)
        .sum()
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(4361, process(input));
    }
}
