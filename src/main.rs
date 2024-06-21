use std::{io::Write, time};

fn main() {
    for _ in 0..10 {
        println!("Solving...");
        solve_brute_force();
    }
}

fn index_to_row_column(index: usize) -> (usize, usize) {
    (index / 9, index % 9)
}

fn row_column_to_index(row: usize, column: usize) -> usize {
    row * 9 + column
}

fn collect_input() -> [i32; 81] {
    let mut new_board = [0; 81];

    new_board[3] = 8;
    new_board[5] = 1;
    new_board[16] = 4;
    new_board[17] = 3;
    new_board[18] = 5;
    new_board[31] = 7;
    new_board[33] = 9;
    new_board[42] = 1;
    new_board[46] = 2;
    new_board[49] = 3;
    new_board[54] = 6;
    new_board[61] = 7;
    new_board[62] = 5;
    new_board[65] = 3;
    new_board[66] = 4;
    new_board[75] = 2;
    new_board[78] = 6;

    return new_board;
}

fn solve_brute_force() {
    let board = collect_input();
    let mut backtrack = 0;
    let time = time::Instant::now();

    let solution_found = solve(board, 0, &mut backtrack);
    let solution_time = time.elapsed();

    println!("Solution found: {}", solution_found);
    println!("Backtrack count: {}", backtrack);
    println!("Solution time: {}ms", solution_time.as_millis());

    write_solve_time_to_file(solution_time)
}

// Write the solve time to a file comma separated
fn write_solve_time_to_file(solution_time: time::Duration) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("solve_time.txt")
        .unwrap();

    write!(file, "{}, ", solution_time.as_millis()).unwrap();

    file.flush().unwrap();
}

fn get_possible_moves(board: [i32; 81], index: usize) -> Vec<i32> {
    let mut moves = Vec::new();
    for i in 1..=9 {
        if acceptable(board, index, i) {
            moves.push(i);
        }
    }
    return moves;
}

fn acceptable(board: [i32; 81], index: usize, value: i32) -> bool {
    let (row, column) = index_to_row_column(index);

    // if already present on the column, not acceptable
    for i in 0..9 {
        let board_index = row_column_to_index(row, i);
        if board[board_index] == value {
            return false;
        }
    }

    // if already present on the row, not acceptable
    for i in 0..9 {
        let board_index = row_column_to_index(i, column);
        if board[board_index] == value {
            return false;
        }
    }

    // if already present in the 3x3 grid, not acceptable
    let r1 = (row / 3) * 3;
    let c1 = (column / 3) * 3;
    for r in r1..(r1 + 3) {
        for c in c1..(c1 + 3) {
            let board_index = row_column_to_index(r, c);
            if board[board_index] == value {
                return false;
            }
        }
    }

    // acceptable
    return true;
}

fn solve(mut board: [i32; 81], mut index: usize, backtrack: &mut i32) -> bool {
    while (index < board.len()) && (board[index] != 0) {
        index += 1;
    }
    if index >= board.len() {
        return true;
    }
    let moves = get_possible_moves(board, index);
    for move_ in moves {
        board[index] = move_;
        if solve(board, index + 1, backtrack) {
            return true;
        }
    }
    board[index] = 0;
    *backtrack += 1;

    return false;
}
