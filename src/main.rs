mod constants;
mod paddle;
mod ball;
mod collision;
mod game;

use constants::*;
use macroquad::prelude::*;
use crate::game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pong".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    
    loop {
        let dt = get_frame_time().min(0.05);
        
        // Check collisions

        // 3. update                                                                                                                                                                                                                                                                                                  
        // move things, check collisions, update score, etc.
        game.update();
        
        // 4. draw  
        clear_background(BLACK);
        // draw everything here
        game.pl.draw();
        game.pr.draw();
        game.ball.draw();

        next_frame().await;          // 5. present frame — always last
    }
}
