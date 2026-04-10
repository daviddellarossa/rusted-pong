use macroquad::input::{is_key_down, KeyCode};
use macroquad::time::get_frame_time;
use crate::ball::Ball;
use crate::collision::{check_paddle_bounce, check_wall_bounce};
use crate::constants::*;
use crate::paddle::{Paddle, Side};

#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    StartScreen,
    Playing,
    Scored { pause_timer: f32 },
    GameOver { winner: u32 },
}

pub struct Game {
    pub pl: Paddle,
    pub pr: Paddle,
    pub ball: Ball,
    pub score_l: u32,
    pub score_r: u32,
    pub phase: GamePhase,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pl: Paddle::new(Side::Left),
            pr: Paddle::new(Side::Right),
            ball: Ball::new(),
            score_l: 0,
            score_r: 0,
            phase: GamePhase::StartScreen,
        }
    }
    
    pub fn update(&mut self) {
        let dt = get_frame_time().min(0.05);
        
        match self.phase {
            GamePhase::StartScreen => {
                if is_key_down(KeyCode::Space) {
                    self.phase = GamePhase::Playing;
                }
            }
            GamePhase::Playing => {
                // input
                if is_key_down(KeyCode::W) { self.pl.move_up(dt)}
                if is_key_down(KeyCode::S) { self.pl.move_down(dt)}
                if is_key_down(KeyCode::Up) { self.pr.move_up(dt)}
                if is_key_down(KeyCode::Down) { self.pr.move_down(dt)}
                self.pl.clamp_to_screen();
                self.pr.clamp_to_screen();
                
                // physics
                self.ball.update(dt);
                check_wall_bounce(&mut self.ball);
                check_paddle_bounce(&mut self.ball, &self.pl, &self.pr);
                
                // scoring
                if self.ball.rect.x + self.ball.rect.w < 0.0 {
                    self.score_r += 1;
                    self.ball.reset(1.0);
                    self.phase = GamePhase::Scored { pause_timer: 0.0 };
                } else if self.ball.rect.x > SCREEN_WIDTH as f32 {
                    self.score_l += 1;
                    self.ball.reset(-1.0);
                    self.phase = GamePhase::Scored { pause_timer: 0.0 };
                } 
                
                // win detection
                if self.score_l == WINNING_SCORE {
                    self.phase = GamePhase::GameOver { winner: 1 };
                } else if self.score_r == 3 {
                    self.phase = GamePhase::GameOver { winner: 2 };
                }
            }
            GamePhase::Scored { ref mut pause_timer } => {
                *pause_timer -= dt;
                if *pause_timer <= 0.0 {
                    self.phase = GamePhase::Playing;
                }
            }
            GamePhase::GameOver { .. } => {}
        }
    }
}