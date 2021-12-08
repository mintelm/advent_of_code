fn get_nth_common_bit(data: &str, n: usize) -> String {
    let one_count = data.lines()
        .map(|x| x.chars().nth(n).unwrap())
        .filter(|x| *x == '1')
        .count();
    let zero_count = data.lines().count() - one_count;

    if one_count > zero_count { "1".to_string() } else { "0".to_string() }
}

fn main() {
    let data = include_str!("../input.txt");
    let bit_len = data.lines().next().unwrap().len();

    let mut gamma_rate_bit_string: String = "".to_owned();

    for i in 0..bit_len {
        gamma_rate_bit_string.push_str(&get_nth_common_bit(&data, i));
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate_bit_string, 2).unwrap();
    let epsilon_rate = (!gamma_rate) ^ (isize::max_value() << bit_len);

    println!("{}", gamma_rate * epsilon_rate);
}
