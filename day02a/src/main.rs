fn get_nav_value(data: &str, direction: &str) -> isize {
    data.lines().filter(|x| x.starts_with(direction))
        .map(|x| x.trim_start_matches(char::is_alphabetic).trim_start().parse::<isize>().unwrap())
        .sum::<isize>()
}

fn main() {
    let data = include_str!("../input.txt");

    println!(
        "{}",
        get_nav_value(data, "forward") * (get_nav_value(data, "down") - get_nav_value(data, "up"))
    );
}
