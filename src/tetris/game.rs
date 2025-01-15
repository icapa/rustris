use std::{io::{stdout, Write}, time::Duration};


use rand::Rng;

//use crate::{clear_screen, draw_board, draw_floor, draw_score, draw_wall};

/* 
use super::{
    board::{board_delete_piece, board_fix_piece, board_remove_rows, board_set_piece, create_board, does_piece_fit}, draw::{self, clear_screen, draw_board, draw_floor, draw_score, draw_wall}, pieces::{get_piece_width, get_tetris_piece, get_tetris_pieces, rotate_piece}, score::score_calculator
    
};
*/
use crossterm::{
    //cursor::{Hide, MoveTo}, 
    event::{poll, read, Event::Key, KeyCode::{self, Char}}, 
    //execute, 
    //style::{Color, Print, SetBackgroundColor}, 
    terminal::{disable_raw_mode, enable_raw_mode}
};

use super::{board::Board, draw::{self, clear_screen, draw_board, draw_floor, draw_score, draw_wall}, pieces::{get_piece_width, get_tetris_piece, get_tetris_pieces, rotate_piece}, score::score_calculator};

const REF_SPEED:i32 = 20;
const BOARD_HEIGHT: u16 = 10;
const BOARD_WIDTH: u16 = 10;

pub struct Game{
   width:u16,
   height:u16,
}

impl Game{
    pub fn new() -> Game {
        Game{
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        }
    }
    pub fn run(&self){
        let level: i32 = 10;
        let mut speed = REF_SPEED-level;
        let mut score = 0;
        let mut actual_x = 0;
        let mut actual_y = 0;

        //let mut fix_piece=false;
        enable_raw_mode().unwrap();

        let mut stdout = stdout();
        
        let pieces = get_tetris_pieces();
        let mut the_board = Board::new(10,10);

        // Limpia la pantalla
        draw::clear_screen(&mut stdout);
        
        let random_number = rand::thread_rng().gen_range(1..=pieces.len());
        let mut piece = get_tetris_piece(random_number - 1);
        if the_board.does_piece_fit(actual_x, actual_y, &piece){
            the_board.board_set_piece(actual_x, actual_y, &piece);
            draw_board(&mut stdout, &the_board.board);
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
                            the_board.board_delete_piece(actual_x,actual_y,&piece);
                            piece = rotate_piece(&piece);
                            if the_board.does_piece_fit(actual_x, actual_y, &piece){
                                the_board.board_set_piece(actual_x, actual_y, &piece);
                            }
                            
                        },
                        KeyCode::Right => {
                            if actual_x + get_piece_width(&piece) as u16 == 10{
                                continue;
                            }

                            the_board.board_delete_piece(actual_x,actual_y,&piece);
                            if the_board.does_piece_fit(actual_x+1, actual_y, &piece){
                                actual_x+=1;
                            }
                            the_board.board_set_piece(actual_x, actual_y, &piece);
                        },
                        KeyCode::Left => {
                            if actual_x == 0{
                                continue;
                            }
                            the_board.board_delete_piece(actual_x,actual_y,&piece);
                            if the_board.does_piece_fit(actual_x-1, actual_y, &piece){    
                                actual_x-=1;
                            }
                            the_board.board_set_piece(actual_x, actual_y, &piece);
                        },
                        KeyCode::Down => {      
                            the_board.board_delete_piece(actual_x,actual_y,&piece);
                            if the_board.does_piece_fit(actual_x, actual_y+1, &piece){
                                actual_y+=1;
                            }
                            the_board.board_set_piece(actual_x, actual_y, &piece);       
                        },
                        _ => {}
                    }
                    draw_board(&mut stdout, &the_board.board);
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
                        the_board.board_delete_piece(actual_x,actual_y,&piece);
                        if the_board.does_piece_fit(actual_x, actual_y+1, &piece){
                            actual_y+=1;
                            the_board.board_set_piece(actual_x, actual_y, &piece);
                        }
                        else{
                            if actual_y == 0{
                                break;
                            }
                            the_board.board_fix_piece(actual_x, actual_y, &piece);
                            actual_x=5;
                            actual_y=0;
                            score+=score_calculator(the_board.board_remove_rows(),0);
                            let random_number = rand::thread_rng().gen_range(1..=pieces.len());
                            piece = get_tetris_piece(random_number - 1);
                        }   
                    draw_board(&mut stdout, &the_board.board);
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
}