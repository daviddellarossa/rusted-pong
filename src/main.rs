mod constants;
mod paddle;
mod ball;
mod collision;

use constants::*;
use macroquad::prelude::*;
use paddle::*;
use ball::*;


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
    let mut pl = Paddle::new(Side::Left);
    let mut pr = Paddle::new(Side::Right);
    let mut ball = Ball::new();
    
    loop {
        let dt = get_frame_time().min(0.05);   // 1. timing                                                                                                                                                                                                                                                                     

        // 2. input                                                                                                                                                                                                                                                                                                   
        // read keys, mouse, etc.  
        if is_key_down(KeyCode::W) { pl.move_up(dt)}
        if is_key_down(KeyCode::S) { pl.move_down(dt)}
        if is_key_down(KeyCode::Up) { pr.move_up(dt)}
        if is_key_down(KeyCode::Down) { pr.move_down(dt)}
        pl.clamp_to_screen();
        pr.clamp_to_screen();
        
        // Check collisions
        
        collision::check_wall_bounce(&mut ball);
        collision::check_paddle_bounce(&mut ball, &pl, &pr);

        // 3. update                                                                                                                                                                                                                                                                                                  
        // move things, check collisions, update score, etc.
        ball.update(dt);

        // 4. draw  
        clear_background(BLACK);
        // draw everything here
        // draw_rectangle(20.0, 260.0, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        pl.draw();
        pr.draw();
        ball.draw();

        next_frame().await;          // 5. present frame — always last

    }
}
