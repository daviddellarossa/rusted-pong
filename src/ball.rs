use std::sync::Mutex;
use macroquad::prelude::*;
use crate::constants::*;

pub  struct Ball {
    pub rect: Rect,
    pub velocity: Vec2,
    pub speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        let x = SCREEN_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0;
        let y = SCREEN_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0;
        Ball {
            rect: Rect::new(x, y, BALL_SIZE, BALL_SIZE),
            velocity: Vec2::new(BALL_INITIAL_SPEED, BALL_INITIAL_SPEED),
            speed: BALL_INITIAL_SPEED,
        }
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }
    
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.velocity.x * dt;
        self.rect.y += self.velocity.y * dt;
    }
    
    pub fn reset(&mut self, x_dir: f32) {
        self.rect.x = SCREEN_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = SCREEN_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0;
        self.speed = BALL_INITIAL_SPEED;
        self.velocity = Vec2::new(x_dir * self.speed, self.speed * 0.5);
    }
}