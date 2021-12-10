const DAYS: usize = 80;

fn main() {
    let mut lanternfish: Vec<u8> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..DAYS {
        for i in 0..lanternfish.len() {
            if lanternfish[i] != 0 {
                lanternfish[i] -= 1;
            } else {
                lanternfish[i] = 6;
                lanternfish.push(8);
            }
        }
    }

    println!("{:?}", lanternfish.len());
}
