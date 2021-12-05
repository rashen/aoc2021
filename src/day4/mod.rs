type BoardView = [[u8; 5]; 5];

struct Board {
    data: BoardView,
}

impl Board {
    fn from_matrix(matrix: BoardView) -> Self {
        Self { data: matrix }
    }

    fn rows(&self) -> &BoardView {
        &self.data
    }

    fn columns(&self) -> &BoardView {
        todo!()
    }
}

fn find_horizontal_bingo(boards: &[Board], inputs: &[u8]) -> Option<usize> {
    let mut board_index = 0;
    for board in boards {
        for row in board.rows() {
            let mut n_matches = 0;
            for v in row {
                if inputs.contains(v) {
                    n_matches += 1;
                }
            }
            if n_matches >= 5 {
                return Some(board_index);
            }
        }
        board_index += 1;
    }
    None
}

// fn find_vertical_bingo(boards, &[Board], inputs: &[u8]) -> Option<usize> {
// let mut board_index = 0;
// for board in boards {

// }
// }

mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn get_boards() -> Vec<Board> {
        let mut boards = Vec::new();
        boards.push(Board::from_matrix([
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]));
        boards.push(Board::from_matrix([
            [3, 15, 0, 0, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ]));
        boards.push(Board::from_matrix([
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]));
        boards
    }

    fn get_inputs() -> Vec<u8> {
        vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1, 7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
            20, 8, 19, 3, 26, 1,
        ]
    }

    #[test]
    fn test_get_horizontal_bingo() {
        let boards = get_boards();
        let inputs = [3, 15, 0, 0, 22];
        assert_eq!(find_horizontal_bingo(&boards, &inputs), Some(1));
    }
}
