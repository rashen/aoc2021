use std::{fs, io::BufRead, io::BufReader};

type BoardView = [[u8; 5]; 5];

#[derive(Debug, PartialEq, Clone, Copy)]
struct Board {
    rows: BoardView,
    columns: BoardView,
}

impl Board {
    fn from_matrix(matrix: BoardView) -> Self {
        let mut columns: BoardView = [[0; 5]; 5];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                columns[i][j] = matrix[j][i];
            }
        }
        Self {
            rows: matrix,
            columns,
        }
    }

    fn from_values(values: &[u8]) -> Self {
        assert_eq!(values.len(), 25);

        let mut rows: BoardView = [[0; 5]; 5];
        for (row_idx, row) in values.chunks(5).enumerate() {
            for (column_idx, v) in row.iter().enumerate() {
                rows[row_idx][column_idx] = *v;
            }
        }
        Self::from_matrix(rows)
    }

    fn rows(&self) -> &BoardView {
        &self.rows
    }

    fn columns(&self) -> &BoardView {
        &self.columns
    }

    fn values(&self) -> Vec<u8> {
        self.rows.iter().flatten().map(|x| *x).collect()
    }
}

fn read_input() -> (Vec<u8>, Vec<Board>) {
    let filename = "src/day4/input";
    let file = fs::File::open(filename).unwrap();
    let mut contents = BufReader::new(file);
    let mut string_buffer = String::new();
    contents
        .read_line(&mut string_buffer)
        .expect("Failed to read inputs");
    let first_line = string_buffer
        .split(',')
        .filter_map(|r| r.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    string_buffer.clear();

    let mut boards = Vec::new();
    while let Ok(bytes_read) = contents.read_line(&mut string_buffer) {
        if bytes_read == 1 {
            let parsed_matrix = string_buffer
                .split_whitespace()
                .filter_map(|r| r.parse::<u8>().ok())
                .collect::<Vec<u8>>();

            if !parsed_matrix.is_empty() {
                boards.push(Board::from_values(&parsed_matrix));
            }
            string_buffer.clear();
        } else if bytes_read == 0 {
            break;
        }
    }
    assert_eq!(boards.len(), 100);
    (first_line, boards)
}

fn find_horizontal_bingo(boards: &[Board], inputs: &[u8]) -> Option<usize> {
    let mut board_index = 0;
    for board in boards {
        for row in board.rows() {
            if has_bingo(row, inputs) {
                return Some(board_index);
            }
        }
        board_index += 1;
    }
    None
}

fn has_bingo(array: &[u8; 5], inputs: &[u8]) -> bool {
    let mut n_matches = 0;
    for v in array {
        if inputs.contains(v) {
            n_matches += 1;
        }
    }
    if n_matches >= 5 {
        return true;
    }
    false
}

fn find_vertical_bingo(boards: &[Board], inputs: &[u8]) -> Option<usize> {
    let mut board_index = 0;
    for board in boards {
        for columns in board.columns() {
            if has_bingo(columns, inputs) {
                return Some(board_index);
            }
        }
        board_index += 1;
    }
    None
}

pub fn run() {
    let (inputs, boards) = read_input();
    let bingo = find_first_bingo(&boards, &inputs);
    if bingo.is_none() {
        panic!("No bingo");
    }
    let bingo = bingo.unwrap();
    let answer = calculate_answer(&boards[bingo.board_index], &inputs[0..=bingo.input_index]);
    println!("Task1: Final score is {}", answer);

    let bingo = find_last_bingo(&boards, &inputs).unwrap();
    let answer = calculate_answer(&boards[bingo.board_index], &inputs[0..=bingo.input_index]);
    println!("Task2: Final score is {}", answer);
}

#[derive(PartialEq, Debug, Default)]
struct Indices {
    board_index: usize,
    input_index: usize,
}

fn find_first_bingo(boards: &[Board], inputs: &[u8]) -> Option<Indices> {
    for last_index in 4..inputs.len() {
        let current_inputs = &inputs[0..=last_index];
        if let Some(winning_board_index) = find_horizontal_bingo(boards, current_inputs) {
            return Some(Indices {
                board_index: winning_board_index,
                input_index: last_index,
            });
        }
        if let Some(winning_board_index) = find_vertical_bingo(boards, current_inputs) {
            return Some(Indices {
                board_index: winning_board_index,
                input_index: last_index,
            });
        }
    }
    None
}

fn calculate_answer(board: &Board, inputs: &[u8]) -> u32 {
    let sum_of_unmarked_numers = board
        .values()
        .iter()
        .filter(|x| !inputs.contains(x))
        .fold(0_u32, |acc, x| acc + *x as u32);

    sum_of_unmarked_numers * *inputs.last().unwrap() as u32
}

fn find_last_bingo(boards: &[Board], inputs: &[u8]) -> Option<Indices> {
    let mut boards_without_bingo = boards.iter().map(|b| *b).collect::<Vec<Board>>();

    while boards_without_bingo.len() > 1 {
        if let Some(idx) = find_first_bingo(&boards_without_bingo, &inputs) {
            boards_without_bingo.remove(idx.board_index);
        }
    }

    let board_index = boards.iter().position(|b| b == &boards_without_bingo[0]);

    let input_index = find_first_bingo(
        &boards[board_index.unwrap()..=board_index.unwrap()],
        &inputs,
    )
    .unwrap()
    .input_index;

    Some(Indices {
        board_index: board_index.unwrap(),
        input_index,
    })
}

#[cfg(test)]
mod tests {
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
            3, 26, 1,
        ]
    }

    #[test]
    fn test_get_horizontal_bingo() {
        let boards = get_boards();
        let inputs = [3, 15, 0, 0, 22];
        assert_eq!(find_horizontal_bingo(&boards, &inputs), Some(1));
    }

    #[test]
    fn test_get_columns() {
        let boards = get_boards();
        assert_eq!(boards[0].columns()[0], [22, 8, 21, 6, 1])
    }

    #[test]
    fn test_get_vertical_bingo() {
        let boards = get_boards();
        let inputs = [13, 2, 9, 10, 12];
        assert_eq!(find_vertical_bingo(&boards, &inputs), Some(0));
    }

    #[test]
    fn test_get_first_bingo() {
        let boards = get_boards();
        let inputs = get_inputs();
        let indices = find_first_bingo(&boards, &inputs).unwrap();
        assert_eq!(indices.board_index, 2);
        assert_eq!(indices.input_index, 11);
        assert_eq!(inputs[indices.input_index], 24);
    }

    #[test]
    fn test_calculate_answer() {
        let boards = get_boards();
        let inputs = get_inputs();
        assert_eq!(calculate_answer(&boards[2], &inputs[0..12]), 4512);
        assert_eq!(calculate_answer(&boards[1], &inputs[0..15]), 1924);
    }

    #[test]
    fn test_board_from_values() {
        let boards_from_matrix = get_boards();
        let values = [
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        assert_eq!(values.len(), 25);
        let board_from_value = Board::from_values(&values);

        assert_eq!(boards_from_matrix[0], board_from_value);
    }

    #[test]
    fn test_find_last_bingo() {
        let boards = get_boards();
        let inputs = get_inputs();
        assert_eq!(
            find_last_bingo(&boards, &inputs),
            Some(Indices {
                board_index: 1,
                input_index: 14
            })
        );
        assert_eq!(inputs[14], 13);
    }
}
