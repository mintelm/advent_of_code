fn get_nth_common_bit(data: Vec<&str>, n: usize, inverted: bool) -> String {
    let one_count = data.iter()
        .map(|x| x.chars().nth(n).unwrap())
        .filter(|x| *x == '1')
        .count();
    let zero_count = data.len() - one_count;
    let predicate = if inverted {one_count < zero_count } else { one_count >= zero_count };

    if predicate { "1".to_string() } else { "0".to_string() }
}

fn get_rating(data: &str, inverted: bool) -> isize {
    let mut rating_string: String = "".to_owned();
    let mut i = 0;

    loop {
        let filtered_data: Vec<&str> = data.lines().filter(|x| x.starts_with(&rating_string)).collect();
        if filtered_data.len() <= 1 {
            rating_string = filtered_data[0].to_string();
            break;
        }

        rating_string.push_str(&get_nth_common_bit(filtered_data, i, inverted));
        i += 1;
    }
    isize::from_str_radix(&rating_string, 2).unwrap()
}

fn main() {
    let data = include_str!("../input.txt");

    let ox_gen_rating = get_rating(data, false);
    let co2_scrub_rating = get_rating(data, true);

    println!("{}", ox_gen_rating * co2_scrub_rating)
}
