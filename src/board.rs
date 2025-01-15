const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 10;

pub fn create_board() -> Vec<Vec<char>> {
    let mut board = vec![vec![' '; BOARD_WIDTH]; BOARD_HEIGHT];
    board
}

pub fn does_piece_fit(x: u16, y: u16, piece: &[Vec<char>], board: &Vec<Vec<char>>) -> bool {
    for (row_index, row) in piece.iter().enumerate() {
        for (col_index, &block) in row.iter().enumerate() {
            if block == '#' {
                let board_row = row_index as u16 + y;
                let board_col = col_index as u16 + x;
                if board_row >= BOARD_HEIGHT as u16 || board_col >= BOARD_WIDTH as u16 {
                    return false;
                }
                if board[board_row as usize][board_col as usize] != ' ' {
                    return false;
                }
            }
        }
    }
    true
}
pub fn board_set_piece(x: u16, y: u16, piece: &[Vec<char>], board: &mut Vec<Vec<char>>) {
    for (row_index, row) in piece.iter().enumerate() {
        for (col_index, &block) in row.iter().enumerate() {
            if block == '#' {
                let board_row = row_index as u16 + y;
                let board_col = col_index as u16 + x;
                board[board_row as usize][board_col as usize] = block;
            }
        }
    }
}
pub fn board_fix_piece(x: u16, y: u16, piece: &[Vec<char>], board: &mut Vec<Vec<char>>) {
    for (row_index, row) in piece.iter().enumerate() {
        for (col_index, &block) in row.iter().enumerate() {
            if block == '#' {
                let board_row = row_index as u16 + y;
                let board_col = col_index as u16 + x;
                board[board_row as usize][board_col as usize] = '*';
            }
        }
    }
}
pub fn board_delete_piece(x: u16, y: u16, piece: &[Vec<char>], board: &mut Vec<Vec<char>>) {
    for (row_index, row) in piece.iter().enumerate() {
        for (col_index, &block) in row.iter().enumerate() {
            if block == '#' {
                let board_row = row_index as u16 + y;
                let board_col = col_index as u16 + x;
                board[board_row as usize][board_col as usize] = ' ';
            }
        }
    }
}
pub fn board_rows_completed(board: &Vec<Vec<char>>) -> Vec<u16> {
    let mut completed_rows = vec![];
    for (i,row) in board.iter().rev().enumerate() {
        if row.iter().all(|&cell| cell != ' ') {
            completed_rows.push(board.len() as u16 - i as u16 - 1);
        }
    }
    completed_rows.reverse();
    completed_rows
       
}

pub fn board_remove_rows(board: &mut Vec<Vec<char>>) -> u32{
    let completed_rows = board_rows_completed(board);
    for &row in completed_rows.iter() {
        board.remove(row as usize);
        board.insert(0, vec![' '; BOARD_WIDTH]);
    }
    return completed_rows.len() as u32;
}   

// Help functions
#[allow(dead_code)]
pub fn debug_draw_board(board: &Vec<Vec<char>>, aux_text: &str) {
    println!("========== BOARD {} ==========", aux_text);
    for (_row_index, row) in board.iter().enumerate() {
        for (_col_index, &cell) in row.iter().enumerate() {
            if cell == '#' {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("===============================");
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::pieces;

    use super::*;
    

    #[test]
    fn test_create_board_dimensions() {
        let board = create_board();
        assert_eq!(board.len(), BOARD_HEIGHT);
        for row in board.iter() {
            assert_eq!(row.len(), BOARD_WIDTH);
        }
    }

    #[test]
    fn test_create_board_initial_values() {
        let board = create_board();
        for row in board.iter() {
            for &cell in row.iter() {
                assert_eq!(cell, ' ');
            }
        }
    }
    #[test]
    fn test_does_piece_fit_y() {
        let mut board = create_board();
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[0];
        assert!(does_piece_fit(0, 0, piece, &board));
        board_set_piece(0, 0, piece, &mut board);
        
        assert!(!does_piece_fit(0, 0, piece, &board));  
        board_set_piece(2,0, piece, &mut board);
        assert!(!does_piece_fit(2,0, piece, &board));  
        assert!(!does_piece_fit(1, 0, piece, &board));  
        board_set_piece(4, 0,piece, &mut board);
        board_set_piece(6,0, piece, &mut board);
        board_set_piece(8,0, piece, &mut board);
        board_set_piece(0,2, piece, &mut board);
        board_set_piece(0,4, piece, &mut board);
        assert!(!does_piece_fit(0, 9, piece, &board)); 
        assert!(does_piece_fit(0, 8, piece, &board));
    }
    #[test]
    fn test_does_piece_fit_x(){
        let mut board = create_board();
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[0];
        assert!(does_piece_fit(0, 0, &piece, &board));
        board_set_piece(0, 0, &piece, &mut board);
        debug_draw_board(&board,"Hola");
        board_delete_piece(0, 0, &piece, &mut board);
        assert!(does_piece_fit(1, 0, &piece, &board));
        board_set_piece(1, 0, &piece, &mut board);
        debug_draw_board(&board,"Hola");
    }
    #[test]
    fn test_board_rows_completed() {
        let mut board = create_board();
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[0];
        board_set_piece(0, 8, piece, &mut board);
        board_set_piece(2,8, piece, &mut board);
        board_set_piece(4,8, piece, &mut board);
        board_set_piece(6, 8, piece, &mut board);
        board_set_piece(8, 8, piece, &mut board);

        board_set_piece(4, 6, piece, &mut board);   

        debug_draw_board(&board, "INI");
        assert_eq!(board_rows_completed(&board), vec![8,9]);
        board_remove_rows(&mut board);
        debug_draw_board(&board,"ROW");

    }   
    #[test]
    fn test_complete_movement(){
        let mut board = create_board();
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[2];
        let actual_x:u16=0;
        let mut actual_y:u16=0;
        if does_piece_fit(actual_x, actual_y, piece, &board){
            board_set_piece(actual_x,actual_y, &piece, &mut board);
            debug_draw_board(&board,actual_y.to_string().as_str()); 
        }
    
        board_delete_piece(actual_x, actual_y, piece, &mut board);
        
        if does_piece_fit(actual_x, actual_y+1, piece, &board){
            actual_y+=1;
            board_set_piece(actual_x, actual_y, piece, &mut board);
        }else{
            board_set_piece(actual_x, actual_y, piece, &mut board);
        }
        debug_draw_board(&board,actual_y.to_string().as_str());

        board_delete_piece(actual_x, actual_y, piece, &mut board);
       

        if does_piece_fit(actual_x, actual_y+1, piece, &board){
            actual_y+=1;
            board_set_piece(actual_x, actual_y, piece, &mut board);
        }else{
            board_set_piece(actual_x, actual_y, piece, &mut board);
        }

        debug_draw_board(&board,actual_y.to_string().as_str());
        
        board_delete_piece(actual_x, actual_y, piece, &mut board);
        if does_piece_fit(actual_x, actual_y+1, piece, &board){
            actual_y+=1;
            board_set_piece(actual_x, actual_y, piece, &mut board);
        }else{
            board_set_piece(actual_x, actual_y, piece, &mut board);
        }
        debug_draw_board(&board,actual_y.to_string().as_str());

    }
}
