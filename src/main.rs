mod pieces;
mod board;
mod score;
use pieces::{get_piece_height, get_piece_width, get_tetris_piece, get_tetris_pieces, rotate_piece};
use board::{board_delete_piece, board_fix_piece, board_remove_rows, board_set_piece, create_board, debug_draw_board, does_piece_fit};

use crossterm::{
    cursor::{Hide, MoveTo}, 
    event::{poll, read, Event::Key, KeyCode::{self, Char}}, 
    execute, 
    style::{Color, Print, SetBackgroundColor}, 
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
use score::score_calculator;

use std::{io::{stdout, Write}, time::Duration};
use rand::Rng;


const REF_SPEED:i32 = 20;


fn draw_floor<W: Write>(stdout: &mut W) {
    for x in 0..10 {
        execute!(
            stdout,
            MoveTo(x, 10),
            SetBackgroundColor(Color::Red),
            Print("  "), // Dos espacios para hacer bloques más "cuadrados"
            SetBackgroundColor(Color::Reset)
        ).unwrap();
    }
}
fn draw_wall<W: Write>(stdout: &mut W) {
    for y in 0..11 {
        execute!(
            stdout,
            MoveTo(10, y),
            SetBackgroundColor(Color::Red),
            Print("  "), // Dos espacios para hacer bloques más "cuadrados"
            SetBackgroundColor(Color::Reset)
        ).unwrap();
    }
}
fn draw_score<W: Write>(stdout: &mut W, score: u32) {
    execute!(
        stdout,
        MoveTo(12, 0),
        SetBackgroundColor(Color::Reset),
        Print(format!("Score: {}", score)),
        SetBackgroundColor(Color::Reset)
    ).unwrap();
    
}
// Dibuja una pieza en una posición dada
/* 
fn draw_piece<W: Write>(
    stdout: &mut W,
    piece: &[Vec<char>],
    x: u16,
    y: u16,
    color: Color,
)   {
    for (row_index, row) in piece.iter().enumerate() {
        for (col_index, &block) in row.iter().enumerate() {
            if block == '#' {
                execute!(
                    stdout,
                    MoveTo(x + col_index as u16, y + row_index as u16),
                    SetBackgroundColor(color),
                    Print(" "), // Dos espacios para hacer bloques más "cuadrados"
                    SetBackgroundColor(Color::Reset)
                ).unwrap();
            }
        }
    }
    
}
*/
pub fn draw_board<W: Write>(stdout: &mut W, board: &Vec<Vec<char>>) {
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell == '#' || cell == '*' {
                execute!(
                    stdout,
                    MoveTo(col_index as u16, row_index as u16),
                    SetBackgroundColor(Color::Blue),
                    Print(" "), // Dos espacios para hacer bloques más "cuadrados"
                    SetBackgroundColor(Color::Reset)
                ).unwrap();
            }
            else{
                execute!(
                    stdout,
                    MoveTo(col_index as u16, row_index as u16),
                    SetBackgroundColor(Color::Reset),
                    Print(" "), // Dos espacios para hacer bloques más "cuadrados"
                    SetBackgroundColor(Color::Reset)
                ).unwrap();
            }
        }
    }
}
// Limpia la pantalla
fn clear_screen<W: Write>(stdout: &mut W)  {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Hide).unwrap();
}


fn main() {
    
    let mut level: i32 = 10;
    let mut speed = REF_SPEED-level;
    let mut score = 0;
    let mut actual_x = 0;
    let mut actual_y = 0;

    let mut fix_piece=false;
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    let pieces = get_tetris_pieces();
    let mut board = create_board();

    // Limpia la pantalla
    clear_screen(&mut stdout);
    
    let random_number = rand::thread_rng().gen_range(1..=pieces.len());
    let mut piece = get_tetris_piece(random_number - 1);
    if does_piece_fit(actual_x, actual_y, &piece, &board){
        board_set_piece(actual_x, actual_y, &piece, &mut board);
        draw_board(&mut stdout, &board);
    }

    loop {
        if poll(Duration::from_millis(100)).unwrap() {  
            match read() {
            Ok(Key(event)) => {
                match event.code{
                    Char('q') | Char('ñ') => {
                        break;
                    },                   
                    Char('r') => {
                        board_delete_piece(actual_x,actual_y,&piece, &mut board);
                        piece = rotate_piece(&piece);
                        if does_piece_fit(actual_x, actual_y, &piece, &board){
                            board_set_piece(actual_x, actual_y, &piece, &mut board);
                        }
                        
                    },
                    KeyCode::Right => {
                        if actual_x + get_piece_width(&piece) as u16 == 10{
                            continue;
                        }

                        board_delete_piece(actual_x,actual_y,&piece, &mut board);
                        if does_piece_fit(actual_x+1, actual_y, &piece, &board){
                            actual_x+=1;
                        }
                        board_set_piece(actual_x, actual_y, &piece, &mut board);
                    },
                    KeyCode::Left => {
                        if actual_x == 0{
                            continue;
                        }
                        board_delete_piece(actual_x,actual_y,&piece, &mut board);
                        if does_piece_fit(actual_x-1, actual_y, &piece, &board){    
                            actual_x-=1;
                        }
                        board_set_piece(actual_x, actual_y, &piece, &mut board);
                    },
                    KeyCode::Down => {      
                        board_delete_piece(actual_x,actual_y,&piece, &mut board);
                        if does_piece_fit(actual_x, actual_y+1, &piece, &board){
                            actual_y+=1;
                        }
                        board_set_piece(actual_x, actual_y, &piece, &mut board);       
                    },
                    _ => {}
                }
                draw_board(&mut stdout, &board);
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
            },
            _ => {} 
            }
        }else{
            /* 
            Event for move the piece
            */
            speed -= 1;
            if speed == 0 {
                speed = REF_SPEED - level;          
                    board_delete_piece(actual_x,actual_y,&piece, &mut board);
                    if does_piece_fit(actual_x, actual_y+1, &piece, &board){
                        actual_y+=1;
                        board_set_piece(actual_x, actual_y, &piece, &mut board);
                    }
                    else{
                        if actual_y == 0{
                            break;
                        }
                        board_fix_piece(actual_x, actual_y, &piece, &mut board);
                        actual_x=5;
                        actual_y=0;
                        score+=score_calculator(board_remove_rows(&mut board),0);
                        let random_number = rand::thread_rng().gen_range(1..=pieces.len());
                        piece = get_tetris_piece(random_number - 1);
                    }   
                draw_board(&mut stdout, &board);
            }
            
        }
    
        draw_floor(&mut stdout);
        draw_wall(&mut stdout);
        draw_score(&mut stdout, score);
    } 
    clear_screen(&mut stdout);
    stdout.flush().unwrap();
    disable_raw_mode().unwrap();
    
}