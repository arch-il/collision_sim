use macroquad::{
    color,
    input::{is_mouse_button_pressed, mouse_position, MouseButton},
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
        const SUB_STEPS: usize = 8;

        let other_balls = self.balls.clone();

        let dt = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            for (i, ball) in self.balls.iter_mut().enumerate() {
                for (other_i, other_ball) in other_balls.iter().enumerate() {
                    if i == other_i {
                        continue;
                    }

                    let vec = ball.pos - other_ball.pos;
                    let len = 2.0 * RADIUS - vec.length();

                    if len > 0.0 {
                        // ! account for this if pos is updated
                        // ball.pos += vec.normalize() * (len / if i < other_i { 2.0 } else { 1.0 });
                        ball.pos += vec.normalize() * len / 2.0;
                        ball.vel = vec.normalize() * other_ball.vel.length();
                    }
                }

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

                // ! somehow this fixes one problem and produces another
                // ! balls bounce from stationary balls and energy is stabe
                // ! but they tend to stick to each other sometimes
                // other_balls[i] = *ball;
            }
        }
    }

    pub fn input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.balls.push(Ball {
                pos: Vec2::new(
                    mouse_position().0 - RECTANGLE.0,
                    mouse_position().1 - RECTANGLE.1,
                ),
                vel: Vec2::ZERO,
            });
        }
    }

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
