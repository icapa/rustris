//use std::cmp::max_by;

pub fn get_tetris_pieces() -> Vec<Vec<Vec<char>>> {
    vec![
        vec![vec!['#', '#'], vec!['#', '#']], // Cuadrado (O)
        vec![vec![' ', '#', ' '], vec!['#', '#', '#']], // T
        vec![vec!['#', '#', ' '], vec![' ', '#', '#']], // S
        vec![vec![' ', '#', '#'], vec!['#', '#', ' ']], // Z
        vec![vec!['#', '#', '#', '#']], // Línea (I)
        vec![vec!['#', ' '], vec!['#', ' '], vec!['#', '#']], // L
        vec![vec![' ', '#'], vec![' ', '#'], vec!['#', '#']], // J
    ]
}
pub fn get_tetris_piece(index: usize) -> Vec<Vec<char>> {
    get_tetris_pieces()[index].clone()
}
pub fn rotate_piece(piece: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_piece = vec![];
    for col in 0..piece[0].len() {
        let mut new_row = vec![];
        for row in (0..piece.len()).rev() {
            new_row.push(piece[row][col]);
        }
        rotated_piece.push(new_row);
    }
    rotated_piece
}

#[allow(dead_code)]
pub fn get_piece_height(piece: &Vec<Vec<char>>) -> u16 {
    piece.len() as u16
}
#[allow(dead_code)]
pub fn get_piece_width(piece: &Vec<Vec<char>>) -> u16 {
    piece.iter().map(
        |row| row.len()
    ).max().unwrap_or(0) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tetris_pieces() {
        let pieces = get_tetris_pieces();
        assert_eq!(pieces.len(), 7);
    }

    #[test]
    fn test_get_tetris_piece() {
        let piece = get_tetris_piece(0);
        assert_eq!(piece, vec![vec!['#', '#'], vec!['#', '#']]);
    }

    #[test]
    fn test_rotate_piece_square() {
        let piece = vec![vec!['#', '#'], vec!['#', '#']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, piece); // Square piece should remain the same
    }

    #[test]
    fn test_rotate_piece_line() {
        let piece = vec![vec!['#', '#', '#', '#']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']]);
    }

    #[test]
    fn test_rotate_piece_t() {
        let piece = vec![vec![' ', '#', ' '], vec!['#', '#', '#']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, vec![vec!['#', ' '], vec!['#', '#'], vec!['#', ' ']]);
    }

    #[test]
    fn test_rotate_piece_s() {
        let piece = vec![vec!['#', '#', ' '], vec![' ', '#', '#']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, vec![vec![' ', '#'], vec!['#', '#'], vec!['#', ' ']]);
    }

    #[test]
    fn test_rotate_piece_z() {
        let piece = vec![vec![' ', '#', '#'], vec!['#', '#', ' ']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, vec![vec!['#', ' '], vec!['#', '#'], vec![' ', '#']]);
    }

    #[test]
    fn test_rotate_piece_l() {
        let piece = vec![vec!['#', ' '], vec!['#', ' '], vec!['#', '#']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, vec![vec!['#', '#', '#'], vec!['#', ' ', ' ']]);
    }

    #[test]
    fn test_rotate_piece_j() {
        let piece = vec![vec![' ', '#'], vec![' ', '#'], vec!['#', '#']];
        let rotated = rotate_piece(&piece);
        assert_eq!(rotated, vec![vec!['#', ' ', ' '], vec!['#', '#', '#']]);
    }
    #[test]
    fn test_piece_size_cube(){
        let mut piece = get_tetris_piece(0);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
    }
    #[test]
    fn test_piece_size_t(){
        let mut piece = get_tetris_piece(1);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
    }
    #[test]
    fn test_piece_size_s(){
        let mut piece = get_tetris_piece(2);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
    }
    #[test]
    fn test_piece_size_z(){
        let mut piece = get_tetris_piece(3);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
    }
    #[test]
    fn test_piece_size_line(){
        let mut piece = get_tetris_piece(4);
        assert_eq!(get_piece_height(&piece), 1);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 4);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 1);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 4);
    }
    #[test]
    fn test_piece_size_l(){
        let mut piece = get_tetris_piece(5);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
    }
    #[test]
    fn test_piece_size_j(){
        let mut piece = get_tetris_piece(6);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 3);
        piece = rotate_piece(&piece);
        assert_eq!(get_piece_height(&piece), 2);
    }
    #[test]
    fn test_piece_width(){
        let piece = get_tetris_piece(0);
        assert_eq!(get_piece_width(&piece), 2);
        let piece = get_tetris_piece(1);
        assert_eq!(get_piece_width(&piece), 3);
        let piece = get_tetris_piece(2);
        assert_eq!(get_piece_width(&piece), 3);
        let piece = get_tetris_piece(3);
        assert_eq!(get_piece_width(&piece), 3);
        let piece = get_tetris_piece(4);
        assert_eq!(get_piece_width(&piece), 4);
        let piece = get_tetris_piece(5);
        assert_eq!(get_piece_width(&piece), 2);
        let piece = get_tetris_piece(6);
        assert_eq!(get_piece_width(&piece), 2);
    }
}
