fn get_nav_value(nav: &str) -> isize {
    nav.trim_start_matches(char::is_alphabetic).trim_start().parse::<isize>().unwrap()
}

fn main() {
    let data = include_str!("../input.txt");
    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;

    data.lines().for_each(|x| {
        match &x[..1] {
           "f" => {
               horizontal += get_nav_value(x);
               depth += aim * get_nav_value(x);
           },
           "d" => aim += get_nav_value(x),
           "u" => aim -= get_nav_value(x),
           _   => println!("error")
        };
    });

    println!(
        "{}",
        horizontal * depth
    );
}
