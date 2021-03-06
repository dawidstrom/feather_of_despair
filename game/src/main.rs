mod game;
mod board;
mod entity;
mod utils;
mod camera;
mod tile;

extern crate piston_window;
#[macro_use] extern crate enum_primitive;

use piston_window::*;
use std::time::*;

fn main() {
    // Setup game.
    let mut game = game::Game::new();
    game.load_map(String::from("default.map"));
//    game.debug.is_active = true;
    
    // Setup piston.
    let window_size = [
        (game.camera.size.width  * game.board.scale) as f64,
        (game.camera.size.height * game.board.scale) as f64
    ];

    let mut window: PistonWindow =
        WindowSettings::new(
            "Feather of Despair", window_size
        ).exit_on_esc(true).build().unwrap();

    let mut now = Instant::now();

    // Main game loop.
    while let Some(event) = window.next() {
        // Handle input.
        if let Some(button_args) = event.button_args() {
            game.on_input(button_args);
        }

        let elapsed = now.elapsed().as_millis();
        now = Instant::now();

        // Update game world.
        game.on_update(elapsed as i64);

        // Main draw loop.
        game.on_render(event, &mut window);
    }
}
