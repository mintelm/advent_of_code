/* We can just compare first and last element of a quadruple to decide if the sum of each
 * three-measurement window is increasing. */

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .windows(4)
            .filter(|x| x[0] < x[3])
            .count()
    );
}
