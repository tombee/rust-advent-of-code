const MAX: [usize; 3] = [
    12, 13, 14
];
fn main() {
    println!(
        "{}",
        process(include_str!("../input.txt"))
    )
}

fn process(input: &str) -> usize {
    return input
        .lines()
        .enumerate()
        .filter_map(|(id, line)| {
            let mut rgb = [0, 0, 0];
            line.splitn(2, ":").nth(1)
                .unwrap()
                .split([',', ';'])
                .all(|item| {
                    let i = match item.split(" ").nth(2).unwrap() {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => unreachable!(),
                    };
                    rgb[i] = rgb[i].max(atoi::atoi(item.split(" ").nth(1).unwrap().as_bytes()).unwrap());
                    rgb[i] <= MAX[i]
                })
                .then_some(id + 1)
        })
        .inspect(|x| println!("{}", x))
        .sum::<usize>();
}
