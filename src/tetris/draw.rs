use std::io::Write;

use crossterm::{cursor::{Hide, MoveTo}, execute, style::{Color, Print, SetBackgroundColor}, terminal::{Clear, ClearType}};

pub fn draw_floor<W: Write>(stdout: &mut W) {
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
pub fn draw_wall<W: Write>(stdout: &mut W) {
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
pub fn draw_score<W: Write>(stdout: &mut W, score: u32) {
    execute!(
        stdout,
        MoveTo(12, 0),
        SetBackgroundColor(Color::Reset),
        Print(format!("Score: {}", score)),
        SetBackgroundColor(Color::Reset)
    ).unwrap();
    
}
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
pub fn clear_screen<W: Write>(stdout: &mut W)  {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0), Hide).unwrap();
}