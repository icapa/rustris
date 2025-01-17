pub struct Board {
    pub width: u16,
    pub height: u16,
    pub board: Vec<Vec<char>>,
}
impl Board{
    pub fn new(width:u16,height:u16) -> Board{
        Board{
            width,
            height,
            //board: create_board(width,height),
            board: vec![vec![' '; width as usize]; height as usize],
        }
    }

    pub fn does_piece_fit(&self,x: u16, y: u16, piece: &[Vec<char>]) -> bool {
        for (row_index, row) in piece.iter().enumerate() {
            for (col_index, &block) in row.iter().enumerate() {
                if block == '#' {
                    let board_row = row_index as u16 + y;
                    let board_col = col_index as u16 + x;
                    if board_row >= self.height  || board_col >= self.width{
                        return false;
                    }
                    if self.board[board_row as usize][board_col as usize] != ' ' {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn board_set_piece(&mut self,x: u16, y: u16, piece: &[Vec<char>]) {
        for (row_index, row) in piece.iter().enumerate() {
            for (col_index, &block) in row.iter().enumerate() {
                if block == '#' {
                    let board_row = row_index as u16 + y;
                    let board_col = col_index as u16 + x;
                    self.board[board_row as usize][board_col as usize] = block;
                }
            }
        }
    }
    pub fn board_fix_piece(&mut self,x: u16, y: u16, piece: &[Vec<char>]) {
        for (row_index, row) in piece.iter().enumerate() {
            for (col_index, &block) in row.iter().enumerate() {
                if block == '#' {
                    let board_row = row_index as u16 + y;
                    let board_col = col_index as u16 + x;
                    self.board[board_row as usize][board_col as usize] = '*';
                }
            }
        }
    }
    pub fn board_delete_piece(&mut self,x: u16, y: u16, piece: &[Vec<char>]) {
        for (row_index, row) in piece.iter().enumerate() {
            for (col_index, &block) in row.iter().enumerate() {
                if block == '#' {
                    let board_row = row_index as u16 + y;
                    let board_col = col_index as u16 + x;
                    self.board[board_row as usize][board_col as usize] = ' ';
                }
            }
        }
    }
    pub fn board_rows_completed(&self) -> Vec<u16> {
        let mut completed_rows = vec![];
        for (i,row) in self.board.iter().rev().enumerate() {
            if row.iter().all(|&cell| cell != ' ') {
                completed_rows.push(self.board.len() as u16 - i as u16 - 1);
            }
        }
        completed_rows.reverse();
        completed_rows
        
    }

    pub fn board_remove_rows(&mut self) -> u32{
        let completed_rows = self.board_rows_completed();
        for &row in completed_rows.iter() {
            self.board.remove(row as usize);
            self.board.insert(0, vec![' '; self.width as usize]);
        }
        return completed_rows.len() as u32;
    }   
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

    use crate::tetris::{board, pieces::{self, rotate_piece}};

    use super::*;
    

    #[test]
    fn test_create_board_dimensions() {
        
        let the_board = Board::new(10, 10);
        assert_eq!(the_board.board.len(), 10);
        for row in the_board.board.iter() {
            assert_eq!(row.len(), 10);
        }
    }

    #[test]
    fn test_create_board_initial_values() {
        let the_board = Board::new(10, 10);
        for row in the_board.board.iter() {
            for &cell in row.iter() {
                assert_eq!(cell, ' ');
            }
        }
    }
    #[test]
    fn test_does_piece_fit_y() {
        let mut the_board = Board::new(10, 10);
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[0];
        assert!(&the_board.does_piece_fit(0, 0, piece));
        the_board.board_set_piece(0, 0, piece);
        
        assert!(!&the_board.does_piece_fit(0, 0, piece));  
        the_board.board_set_piece(2,0, piece);
        assert!(!the_board.does_piece_fit(2,0, piece));  
        assert!(!the_board.does_piece_fit(1, 0, piece));  
        the_board.board_set_piece(4, 0,piece );
        the_board.board_set_piece(6,0, piece);
        the_board.board_set_piece(8,0, piece );
        the_board.board_set_piece(0,2, piece);
        the_board.board_set_piece(0,4, piece);
        assert!(!the_board.does_piece_fit(0, 9, piece)); 
        assert!(the_board.does_piece_fit(0, 8, piece));
    }
    #[test]
    fn test_does_piece_fit_x(){
        let mut the_board = Board::new(10, 10);        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[0];
        assert!(&the_board.does_piece_fit(0, 0, &piece));
        the_board.board_set_piece(0, 0, &piece);
        debug_draw_board(&the_board.board,"Hola");
        the_board.board_delete_piece(0, 0, &piece);
        assert!(the_board.does_piece_fit(1, 0, &piece));
        the_board.board_set_piece(1, 0, &piece);
        debug_draw_board(&the_board.board,"Hola");
    }
    #[test]
    fn test_board_rows_completed() {
        let mut the_board = Board::new(10, 10);
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[0];
        the_board.board_set_piece(0, 8, piece);
        the_board.board_set_piece(2,8, piece);
        the_board.board_set_piece(4,8, piece);
        the_board.board_set_piece(6, 8, piece);
        the_board.board_set_piece(8, 8, piece);

        the_board.board_set_piece(4, 6, piece);   

        debug_draw_board(&the_board.board, "INI");
        assert_eq!(the_board.board_rows_completed(), vec![8 as u16,9 as u16]);
        the_board.board_remove_rows();
        debug_draw_board(&the_board.board,"ROW");

    }   
    #[test]
    fn test_complete_movement(){
        let mut the_board = Board::new(10, 10);        
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[2];
        let actual_x:u16=0;
        let mut actual_y:u16=0;
        if the_board.does_piece_fit(actual_x, actual_y, piece){
            the_board.board_set_piece(actual_x,actual_y, &piece);
            debug_draw_board(&the_board.board,actual_y.to_string().as_str()); 
        }
    
        the_board.board_delete_piece(actual_x, actual_y, piece);
        
        if the_board.does_piece_fit(actual_x, actual_y+1, piece){
            actual_y+=1;
            the_board.board_set_piece(actual_x, actual_y, piece);
        }else{
            the_board.board_set_piece(actual_x, actual_y, piece);
        }
        debug_draw_board(&the_board.board,actual_y.to_string().as_str());

        the_board.board_delete_piece(actual_x, actual_y, piece);
       

        if the_board.does_piece_fit(actual_x, actual_y+1, piece){
            actual_y+=1;
            the_board.board_set_piece(actual_x, actual_y, piece);
        }else{
            the_board.board_set_piece(actual_x, actual_y, piece);
        }

        debug_draw_board(&the_board.board,actual_y.to_string().as_str());
        
        the_board.board_delete_piece(actual_x, actual_y, piece);
        if the_board.does_piece_fit(actual_x, actual_y+1, piece){
            actual_y+=1;
            the_board.board_set_piece(actual_x, actual_y, piece);
        }else{
            the_board.board_set_piece(actual_x, actual_y, piece);
        }
        debug_draw_board(&the_board.board,actual_y.to_string().as_str());

    }
    #[test]
    fn test_board_rotate_border_piece(){
        let mut the_board = Board::new(10, 10);        
        let all_pieces = pieces::get_tetris_pieces();
        let piece = &all_pieces[2];
        let actual_x:u16=7;
        let actual_y:u16=4;
        the_board.board_set_piece(actual_x,actual_y, &piece);
        debug_draw_board(&the_board.board,actual_y.to_string().as_str()); 
        
        let new_piece = rotate_piece(&piece);

        the_board.board_delete_piece(actual_x, actual_y, &piece);
        assert_eq!(true,the_board.does_piece_fit(actual_x, actual_y, &new_piece));
        the_board.board_set_piece(actual_x, actual_y, &new_piece);  
        debug_draw_board(&the_board.board,actual_y.to_string().as_str()); 
        
        let new_piece2 = rotate_piece(&new_piece);
        the_board.board_delete_piece(actual_x, actual_y, &new_piece);
        assert_eq!(true,the_board.does_piece_fit(actual_x, actual_y, &new_piece2));

        the_board.board_set_piece(actual_x, actual_y, &new_piece2);  
        debug_draw_board(&the_board.board,actual_y.to_string().as_str()); 

        let new_piece3 = rotate_piece(&new_piece2);
        the_board.board_delete_piece(actual_x, actual_y, &new_piece2);
        assert_eq!(true,the_board.does_piece_fit(actual_x, actual_y, &new_piece3));

        the_board.board_set_piece(actual_x, actual_y, &new_piece3);  
        debug_draw_board(&the_board.board,actual_y.to_string().as_str()); 




    }
}
