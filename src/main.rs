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

    let mut row_masks = [0u16; 9];
    let mut col_masks = [0u16; 9];
    let mut box_masks = [0u16; 9];

    update_masks(&board, &mut row_masks, &mut col_masks, &mut box_masks);

    let solution_found = solve_with_bitmask(
        board,
        0,
        &mut backtrack,
        &mut row_masks,
        &mut col_masks,
        &mut box_masks,
    );
    let solution_time = time.elapsed();

    println!("Solution found: {}", solution_found);
    println!("Backtrack count: {}", backtrack);
    println!("Solution time: {}ms", solution_time.as_millis());

    write_solve_time_to_file(solution_time)
}

fn update_masks(
    board: &[i32; 81],
    row_masks: &mut [u16; 9],
    col_masks: &mut [u16; 9],
    box_masks: &mut [u16; 9],
) {
    for (index, &value) in board.iter().enumerate() {
        if value != 0 {
            let (row, col) = index_to_row_column(index);
            let box_index = (row / 3) * 3 + col / 3;
            row_masks[row] |= 1 << (value - 1);
            col_masks[col] |= 1 << (value - 1);
            box_masks[box_index] |= 1 << (value - 1);
        }
    }
}

fn solve_with_bitmask(
    mut board: [i32; 81],
    mut index: usize,
    backtrack: &mut i32,
    row_masks: &mut [u16; 9],
    col_masks: &mut [u16; 9],
    box_masks: &mut [u16; 9],
) -> bool {
    while (index < board.len()) && (board[index] != 0) {
        index += 1;
    }
    if index >= board.len() {
        return true;
    }
    let (row, col) = index_to_row_column(index);
    let box_index = (row / 3) * 3 + col / 3;
    for move_ in 1..=9 {
        let mask = 1 << (move_ - 1);
        if (row_masks[row] & mask) == 0
            && (col_masks[col] & mask) == 0
            && (box_masks[box_index] & mask) == 0
        {
            board[index] = move_ as i32;
            row_masks[row] |= mask;
            col_masks[col] |= mask;
            box_masks[box_index] |= mask;
            if solve_with_bitmask(board, index + 1, backtrack, row_masks, col_masks, box_masks) {
                return true;
            }
            row_masks[row] &= !mask;
            col_masks[col] &= !mask;
            box_masks[box_index] &= !mask;
            board[index] = 0;
            *backtrack += 1;
        }
    }
    return false;
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
