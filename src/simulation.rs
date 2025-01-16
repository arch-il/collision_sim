use macroquad::{
    color,
    math::Vec2,
    shapes::{draw_circle, draw_rectangle_lines},
};

const RECTANGLE: (f32, f32, f32, f32) = (25.0, 25.0, 450.0, 450.0);
const RADIUS: f32 = 7.5;

pub struct Simulation {
    balls: Vec<Ball>,
}

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            balls: vec![Ball {
                pos: Vec2::new(225.0, 20.0),
                vel: Vec2::ZERO,
            }],
        }
    }

    pub fn update(&mut self, dt: f32) {}

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
