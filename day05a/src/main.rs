const DIAGRAM_SIZE: usize = 1000;

fn main() {
    let vent_lines: Vec<((u16, u16), (u16, u16))> = include_str!("../input.txt")
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(p1, p2)| {
            (p1.split_once(',').unwrap(), p2.split_once(',').unwrap())
        })
        .map(|((x1, y1), (x2, y2))| {
            let x = (x1.parse().unwrap(), y1.parse().unwrap());
            let y = (x2.parse().unwrap(), y2.parse().unwrap());
            (x, y)
        })
        .collect();

    let mut diagram: [[u8; DIAGRAM_SIZE]; DIAGRAM_SIZE] = [[0; DIAGRAM_SIZE]; DIAGRAM_SIZE];

    for (p1, p2) in vent_lines.iter() {
        // x1 == x2 -> vertical
        if p1.0 == p2.0 {
            let mut y_range = [p1.1, p2.1];
            y_range.sort();

            for i in y_range[0]..=y_range[1] {
                diagram[p1.0 as usize][i as usize] += 1;
            }
        // y1 == y2 -> horizontal
        } else if p1.1 == p2.1 {
            let mut x_range = [p1.0, p2.0];
            x_range.sort();

            for i in x_range[0]..=x_range[1] {
                diagram[i as usize][p1.1 as usize] += 1;
            }
        }
    }

    println!("{}", diagram.iter().flatten().filter(|x| x.gt(&&1)).count());
}
