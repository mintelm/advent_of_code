const DAYS: usize = 80;

fn main() {
    let mut lanternfish: [u128; 9] = [0; 9];

    include_str!("../input.txt")
        .trim()
        .split(',')
        .for_each(|s| {
            let i: usize = s.parse().unwrap();
            lanternfish[i] += 1;
        });

    for day in 1..DAYS {
        lanternfish[(day + 7) % 9] += lanternfish[day % 9];
    }

    println!("{:?}", lanternfish.iter().sum::<u128>());
}
