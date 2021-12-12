use std::cmp;

fn main() {
    let h_pos: Vec<u16> = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mean: f32 = h_pos.iter().map(|x| *x as f32).sum::<f32>() / h_pos.len() as f32;

    let mut count_floor: u128 = 0;
    let mut count_ceil: u128 = 0;

    for i in h_pos {
        let distance_floor = 1..=i32::abs(mean.floor() as i32 - i as i32) as u128;
        let distance_ceil = 1..=i32::abs(mean.ceil() as i32 - i as i32) as u128;
        count_floor += distance_floor.sum::<u128>();
        count_ceil += distance_ceil.sum::<u128>();
    }

    println!("{}", cmp::min(count_ceil, count_floor));
}
