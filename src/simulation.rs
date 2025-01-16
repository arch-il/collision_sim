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

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            balls: vec![Ball {
                pos: Vec2::new(225.0, 400.0),
                vel: Vec2::ZERO,
            }],
        }
    }

    pub fn update(&mut self, dt: f32) {
        const G: f32 = 980.0;
        const STEP_SIZE: f32 = 0.000001;

        for ball in self.balls.iter_mut() {
            for _ in 0..(dt / STEP_SIZE) as usize {
                let dt = STEP_SIZE;

                ball.vel.y += G * dt; // ! negative work of gravity not accounted after bounce
                ball.pos += ball.vel * dt;

                if ball.pos.y >= RECTANGLE.3 - RADIUS {
                    ball.pos.y = 2.0 * (RECTANGLE.3 - RADIUS) - ball.pos.y;
                    ball.vel.y *= -1.0;
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
