mod tetris;
use tetris::game::Game;

fn main() {
    let tetris_game=Game::new();
    tetris_game.run();
}