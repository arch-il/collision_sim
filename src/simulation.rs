use macroquad::{
    color,
    math::Vec2,
    shapes::{draw_circle, draw_rectangle_lines},
};

const RECTANGLE: (f32, f32, f32, f32) = (25.0, 25.0, 450.0, 450.0);
const RADIUS: f32 = 7.5;

pub struct Simulation {
    pub balls: Vec<Ball>,
}

#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Simulation {
    pub fn new() -> Self {
        Self { balls: Vec::new() }
    }

    pub fn update(&mut self, dt: f32) {
        const G: f32 = 980.0;
        const STEP_SIZE: f32 = 0.000001;

        let other_balls = self.balls.clone();

        for (i, ball) in self.balls.iter_mut().enumerate() {
            for _ in 0..(dt / STEP_SIZE) as usize {
                let dt = STEP_SIZE;

                ball.vel.y += G * dt;
                ball.pos += ball.vel * dt;

                if ball.pos.y >= RECTANGLE.3 - RADIUS {
                    // ! too much acceleration is applied over the border
                    // ? too heavy implementation
                    // ? try uncomment if energy is depleating
                    // let extra_y = ball.pos.y - (RECTANGLE.3 - RADIUS);
                    // let extra_t = extra_y / ball.vel.y;
                    // ball.vel.y += 2.0 * G * extra_t;

                    ball.pos.y = 2.0 * (RECTANGLE.3 - RADIUS) - ball.pos.y;
                    ball.vel.y *= -1.0;
                }
                if ball.pos.y <= RADIUS {
                    ball.pos.y = 2.0 * RADIUS - ball.pos.y;
                    ball.vel.y *= -1.0;
                }

                if ball.pos.x >= RECTANGLE.2 - RADIUS {
                    ball.pos.x = 2.0 * (RECTANGLE.2 - RADIUS) - ball.pos.x;
                    ball.vel.x *= -1.0;
                }
                if ball.pos.x <= RADIUS {
                    ball.pos.x = 2.0 * RADIUS - ball.pos.x;
                    ball.vel.x *= -1.0;
                }

                for (other_i, other_ball) in other_balls.iter().enumerate() {
                    if i == other_i {
                        continue;
                    }

                    let vec = other_ball.pos - ball.pos;
                    let len = vec.length();

                    if len <= 2.0 * RADIUS {
                        ball.pos += vec.normalize() * (len / 2.0);
                        ball.vel = Vec2::ZERO; // ! temporary solution
                    }
                }
            }
        }
    }

    pub fn input(&mut self) {}

    pub fn draw(&self) {
        draw_rectangle_lines(
            RECTANGLE.0,
            RECTANGLE.1,
            RECTANGLE.2,
            RECTANGLE.3,
            3.0,
            color::WHITE,
        );

        for ball in self.balls.iter() {
            draw_circle(
                RECTANGLE.0 + ball.pos.x,
                RECTANGLE.1 + ball.pos.y,
                RADIUS,
                color::LIME,
            );
        }
    }
}
