mod constants;
mod paddle;

use constants::*;

use macroquad::prelude::*;

use paddle::*;


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
    let pl = Paddle::new(Side::Left);
    loop {
        // let dt = get_frame_time();   // 1. timing                                                                                                                                                                                                                                                                     

        // 2. input                                                                                                                                                                                                                                                                                                   
        // read keys, mouse, etc.                                                                                                                                                                                                                                                                                     

        // 3. update                                                                                                                                                                                                                                                                                                  
        // move things, check collisions, update score, etc.

        // 4. draw  
        clear_background(BLACK);
        // draw everything here
        // draw_rectangle(20.0, 260.0, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        pl.draw();

        next_frame().await;          // 5. present frame — always last

    }
}
