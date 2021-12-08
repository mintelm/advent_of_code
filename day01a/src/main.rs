fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
        .windows(2)
        .filter(|a| a[0] < a[1])
        .count()
    );
}
