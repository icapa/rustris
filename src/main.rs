mod tetris;
use tetris::game::Game;

fn main() {
    let tetris_game=Game::new(10,10,10,10);
    tetris_game.run();
}