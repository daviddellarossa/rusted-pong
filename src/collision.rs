use crate::{ball::Ball, paddle::{Paddle, Side}, constants::*};

pub fn check_wall_bounce(ball: &mut Ball){
    if ball.rect.y <= 0.0 {
        ball.rect.y = 0.0;
        ball.velocity.y = ball.velocity.y.abs();
    }
    if ball.rect.y + ball.rect.h >= SCREEN_HEIGHT as f32 {
        ball.rect.y = SCREEN_HEIGHT as f32 - ball.rect.h;
        ball.velocity.y = -ball.velocity.y.abs();
    } 
}

pub fn check_paddle_bounce(ball: &mut Ball, pl: &Paddle, pr: &Paddle) {
    for paddle in [pl, pr].iter() {
        if ball.rect.overlaps(&paddle.rect) {
            let paddle_center = paddle.rect.y + paddle.rect.h / 2.0;
            let ball_center = ball.rect.y + ball.rect.h / 2.0;
            let relative_hit = ((ball_center - paddle_center) / (paddle.rect.h / 2.0)).clamp(-1.0, 1.0);
            let bounce_angle = relative_hit * std::f32::consts::FRAC_PI_3;
            let x_dir = match paddle.side {
                Side::Left => 1.0_f32,
                Side::Right => -1.0_f32,
            };
            ball.speed = (ball.speed + BALL_SPEED_INCREMENT).min(MAX_BALL_SPEED);
            ball.velocity.x = x_dir * ball.speed * bounce_angle.cos();
            ball.velocity.y = ball.speed * bounce_angle.sin();
             
            match paddle.side {
                Side::Left => ball.rect.x = paddle.rect.x + paddle.rect.w + 1.0,
                Side::Right => ball.rect.x = paddle.rect.x - ball.rect.w - 1.0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ball::Ball;
    
    #[test]
    fn ball_bounces_off_top_wall(){
        let mut ball = Ball::new();
        ball.rect.y = -1.0;
        ball.velocity.y = 100.0;
        check_wall_bounce(&mut ball);
        assert!(ball.rect.y >= 0.0);
        assert!(ball.velocity.y > 0.0);
    }
    
    #[test]
    fn ball_bounces_off_bottom_wall(){
        let mut ball = Ball::new();
        ball.rect.y = SCREEN_HEIGHT as f32;
        ball.velocity.y = -100.0;
        check_wall_bounce(&mut ball);
        assert!(ball.velocity.y < 0.0);
    }
}   