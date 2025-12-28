use crate::utils::load_puzzle_input;

pub fn solution(input_path: &str) {
    let mut grid_of_paper = Vec::new();

    if let Ok(input) = load_puzzle_input(input_path) {
        for line in input.map_while(Result::ok) {
            let mut row = Vec::with_capacity(line.len());
            for c in line.chars() {
                if c == '@' {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            grid_of_paper.push(row);
        }
    }

    // for row in rolls_of_paper {
    //     println!("{:?}", row);

    // part 1
    let mut num_accessible_rolls: u32 = 0;
    num_accessible_rolls += calculate_accessible_rolls_of_paper(&grid_of_paper);
    println!("Day 4 Part One Answer: {num_accessible_rolls}"); 

    // part 2
    let mut total_rolls_removed: u32 = 0;
    let mut num_removed: u32 = 1;
    while num_removed != 0 {
        num_removed = access_and_remove_paper(&mut grid_of_paper);
        total_rolls_removed += num_removed;
    }
    println!("Day 4 Part Two Answer: {total_rolls_removed}");
}

fn calculate_accessible_rolls_of_paper(grid_of_paper: &Vec<Vec<i32>>) -> u32 {
    let adjacent_transforms: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), 
                                            (0, -1), (0, 1), 
                                            (1, -1), (1, 0), (1, 1)];
    let mut num_accessible_rolls: u32 = 0;
    let num_rows = grid_of_paper.len();
    let row_capacity = grid_of_paper[0].len();

    for row_idx in 0..num_rows {
        for col_idx in 0..row_capacity {
            if grid_of_paper[row_idx][col_idx] == 0 { continue }

            let mut num_adjacent_rolls = 0;
            for (x, y) in adjacent_transforms {
                let new_row_idx = x.strict_add_unsigned(row_idx);
                let new_col_idx = y.strict_add_unsigned(col_idx);

                let row_exceeded = new_row_idx < 0 || new_row_idx >= num_rows.cast_signed();
                let col_exceeded = new_col_idx < 0 || new_col_idx >= row_capacity.cast_signed();

                if row_exceeded || col_exceeded {
                    continue
                }
                num_adjacent_rolls += grid_of_paper[new_row_idx.cast_unsigned()][new_col_idx.cast_unsigned()];
            }

            if num_adjacent_rolls < 4 {
                num_accessible_rolls += 1;
            }
        }
    }
    num_accessible_rolls
}

fn access_and_remove_paper(grid_of_paper: &mut Vec<Vec<i32>>) -> u32 {
    let adjacent_transforms: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), 
                                                (0, -1), (0, 1), 
                                                (1, -1), (1, 0), (1, 1)];
    let mut num_accessible_rolls: u32 = 0;
    let num_rows = grid_of_paper.len();
    let row_capacity = grid_of_paper[0].len();                                            

    for row_idx in 0..num_rows {
        for col_idx in 0..row_capacity {
            if grid_of_paper[row_idx][col_idx] == 0 { continue }

            let mut num_adjacent_rolls = 0;
            for (x, y) in adjacent_transforms {
                let new_row_idx = x.strict_add_unsigned(row_idx);
                let new_col_idx = y.strict_add_unsigned(col_idx);

                let row_exceeded = new_row_idx < 0 || new_row_idx >= num_rows.cast_signed();
                let col_exceeded = new_col_idx < 0 || new_col_idx >= row_capacity.cast_signed();

                if row_exceeded || col_exceeded {
                    continue
                }
                num_adjacent_rolls += grid_of_paper[new_row_idx.cast_unsigned()][new_col_idx.cast_unsigned()];
            }

            if num_adjacent_rolls < 4 {
                num_accessible_rolls += 1;
                grid_of_paper[row_idx][col_idx] = 0;
            }
        }
    }
    num_accessible_rolls
}