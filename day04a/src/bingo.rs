const BOARD_SIZE: usize = 5;

#[derive(Default, Copy, Clone)]
struct Bingofield {
    value: u16,
    marked: bool,
}

#[derive(Default, Copy, Clone)]
pub struct Bingoboard {
    board: [[Bingofield; BOARD_SIZE]; BOARD_SIZE],
    solved: bool,
}

impl Bingoboard {
    pub fn new() -> Self {
        Bingoboard::default()
    }

    pub fn set(&mut self, boardvector: Vec<u16>) {
        if boardvector.len() == BOARD_SIZE.pow(2) {
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    self.board[i][j].value = boardvector[i * BOARD_SIZE + j];
                }
            }
        } else {
            panic!("wrong input format");
        }
    }

    pub fn get_solved(self) -> bool {
        self.solved
    }

    pub fn mark(&mut self, value: u16) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j].value == value {
                    self.board[i][j].marked = true;
                }
            }
        }
    }

    pub fn check_solved(&mut self) -> bool {
        for i in 0..BOARD_SIZE {
            let mut row_solved = true;
            let mut col_solved = true;

            for j in 0..BOARD_SIZE {
                row_solved &= self.board[i][j].marked;
                col_solved &= self.board[j][i].marked;
            }
            self.solved = row_solved || col_solved;
            if self.solved { break; }
        }
        self.solved
    }

    pub fn calc_score(&mut self, last_value: u16) -> usize {
        let mut score = 0;

        if self.solved {
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    if !self.board[i][j].marked {
                        score += self.board[i][j].value as usize;
                    }
                }
            }
        }
        score * last_value as usize
    }
}
