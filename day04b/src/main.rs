mod bingo;

fn main() {
    let data: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let solutions: Vec<u16> =
        data[0].split(',')
            .map(|s| s.parse().unwrap())
            .collect();
    let mut bingoboards: Vec<bingo::Bingoboard> = vec![];
    let mut score: usize = 0;
    let mut solved_boards = 0;

    for line in data.iter().skip(1) {
        let boardvector: Vec<u16> =
            line.split(|c| c == ' ' || c == '\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
        let mut board = bingo::Bingoboard::new();

        board.set(boardvector);
        bingoboards.push(board);
    }

    let board_length = bingoboards.len();

    'outer: for solution in solutions {
        for board in bingoboards.iter_mut() {
            if !board.get_solved(){
                board.mark(solution);
                if board.check_solved() {
                    solved_boards += 1;
                    // check if its our last solved board
                    if solved_boards == board_length {
                        score = board.calc_score(solution);
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("{}", score);
}
