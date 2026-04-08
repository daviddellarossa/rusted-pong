use macroquad::prelude::*;
use crate::constants::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Side { Left, Right}

pub struct Paddle {
    pub rect: Rect,
    pub side: Side,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        let x = match side {
            Side::Left => PADDLE_MARGIN,
            Side::Right => SCREEN_WIDTH as f32 - PADDLE_MARGIN - PADDLE_WIDTH,
        };
        let y = (SCREEN_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0;
        Paddle { rect: Rect::new(x, y, PADDLE_WIDTH, PADDLE_HEIGHT), side}
    }
    
    pub fn move_up(&mut self, dt: f32){
        self.rect.y -= PADDLE_SPEED * dt;
    }
    
    pub fn move_down(&mut self, dt: f32){
        self.rect.y += PADDLE_SPEED * dt;   
    }
    
    pub fn clamp_to_screen(&mut self){
        self.rect.y = self.rect.y.clamp(0.0, SCREEN_HEIGHT as f32 - self.rect.h);
    }
    
    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
}