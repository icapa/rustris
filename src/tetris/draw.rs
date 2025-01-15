use std::io::Write;

use crossterm::{cursor::{Hide, MoveTo}, execute, style::{Color, Print, SetBackgroundColor, SetForegroundColor}, terminal::{Clear, ClearType}};

pub struct Draw <W: Write>{
    offset_x:u16,
    offset_y:u16,
    width:u16,
    height:u16,
    stdout: W,
}
impl <W:Write> Draw<W>{
    pub fn new(offset_x:u16,offset_y:u16,w:u16,h:u16,stdout: W) -> Self {
        Draw{
            offset_x:offset_x,
            offset_y:offset_y,
            width:w,
            height: h,
            stdout: stdout,
        }
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn draw_floor(&mut self) {
        for x in self.offset_x..self.width+self.offset_x+2 {
            execute!(
                self.stdout,
                MoveTo(x, self.offset_y+self.height),
                SetBackgroundColor(Color::DarkBlue),
                Print("  "), // Dos espacios para hacer bloques más "cuadrados"
                SetBackgroundColor(Color::Reset)
            ).unwrap();
            execute!(
                self.stdout,
                MoveTo(x, self.offset_y),
                SetBackgroundColor(Color::DarkBlue),
                Print("  "), // Dos espacios para hacer bloques más "cuadrados"
                SetBackgroundColor(Color::Reset)
            ).unwrap();
        }
    }
    pub fn draw_wall(&mut self) {
        for y in self.offset_y..self.offset_y+self.height+1 {
            execute!(
                self.stdout,
                MoveTo(self.offset_x+self.width+2, y),
                SetBackgroundColor(Color::DarkBlue),
                Print("  "), // Dos espacios para hacer bloques más "cuadrados"
                SetBackgroundColor(Color::Reset)
            ).unwrap();
            execute!(
                self.stdout,
                MoveTo(self.offset_x, y),
                SetBackgroundColor(Color::DarkBlue),
                Print("  "), // Dos espacios para hacer bloques más "cuadrados"
                SetBackgroundColor(Color::Reset)
            ).unwrap();
        }
    }
    pub fn draw_score(&mut self, score: u32) {
        execute!(
            self.stdout,
            MoveTo(self.offset_x+self.width+4, self.offset_y),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Cyan),
            Print(format!("Score: {}", score)),
            SetBackgroundColor(Color::Reset)
        ).unwrap();
        
    }
    pub fn draw_board(&mut self, board: &Vec<Vec<char>>) {
        for (row_index, row) in board.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell == '#' || cell == '*' {
                    execute!(
                        self.stdout,
                        MoveTo((col_index as u16)+self.offset_x+2, (row_index as u16)+self.offset_y),
                        SetBackgroundColor(Color::Blue),
                        Print(" "), // Dos espacios para hacer bloques más "cuadrados"
                        SetBackgroundColor(Color::Reset)
                    ).unwrap();
                }
                else{
                    execute!(
                       self.stdout,
                        MoveTo((col_index as u16)+self.offset_x+2, (row_index as u16)+self.offset_y),
                        SetBackgroundColor(Color::Reset),
                        Print(" "), // Dos espacios para hacer bloques más "cuadrados"
                        SetBackgroundColor(Color::Reset)
                    ).unwrap();
                }
            }
        }
    }
    // Limpia la pantalla
    pub fn clear_screen(&mut self)  {
        execute!(self.stdout, Clear(ClearType::All), MoveTo(self.offset_x, self.offset_y), Hide).unwrap();
    }
}