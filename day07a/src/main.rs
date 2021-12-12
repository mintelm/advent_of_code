fn main() {
    let mut h_pos: Vec<u16> = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    h_pos.sort();
    let median = h_pos[h_pos.len() / 2];
    let mut count = 0;

    for i in h_pos {
        count += i32::abs(median as i32 - i as i32);
    }

    println!("{}", count);
}
