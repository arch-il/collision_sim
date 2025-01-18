use macroquad::{
    color,
    input::{self, is_mouse_button_pressed, mouse_position, MouseButton},
    math::Vec2,
    shapes::{draw_circle, draw_rectangle_lines},
    time,
};

const RECTANGLE: (f32, f32, f32, f32) = (25.0, 25.0, 450.0, 450.0);
const RADIUS: f32 = 2.0;

pub struct Simulation {
    pub balls: Vec<Ball>,
    pub spawner_count: usize,
    elapsed_time: f32,
}

#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            balls: Vec::new(),
            spawner_count: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        const G: f32 = 980.0;
        const SUB_STEPS: usize = 8;

        let dt = dt / SUB_STEPS as f32;

        for _ in 0..SUB_STEPS {
            let rows = (RECTANGLE.3 / (RADIUS + RADIUS)).ceil() as usize;
            let cols = (RECTANGLE.2 / (RADIUS + RADIUS)).ceil() as usize;
            let mut grid = vec![vec![vec![]; cols]; rows];

            for (i, ball) in self.balls.iter_mut().enumerate() {
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

                grid[(ball.pos.x / (RADIUS + RADIUS)) as usize]
                    [(ball.pos.y / (RADIUS + RADIUS)) as usize]
                    .push(i);
            }

            let mut collisions = Vec::new();
            for i in 0..self.balls.len() {
                for j in (i + 1)..self.balls.len() {
                    let vec = self.balls[i].pos - self.balls[j].pos;
                    if vec.length_squared() < (2.0 * RADIUS).powi(2) {
                        collisions.push((i, j));
                    }
                }
            }

            for (a, b) in collisions.into_iter() {
                let impact = self.balls[a].pos - self.balls[b].pos;

                let corr = impact.normalize() * (RADIUS + RADIUS - impact.length()) / 2.0;
                self.balls[a].pos += corr;
                self.balls[b].pos -= corr;

                let pos_diff = self.balls[a].pos - self.balls[b].pos;
                let vel_diff = self.balls[a].vel - self.balls[b].vel;
                let vel_proj = Vec2::dot(vel_diff, pos_diff) / pos_diff.length_squared() * pos_diff;

                self.balls[a].vel -= vel_proj;
                self.balls[b].vel += vel_proj;

                self.balls[a].vel *= 0.99;
                self.balls[b].vel *= 0.99;
            }
        }
    }

    pub fn spawn(&mut self) {
        const BALLS_PER_SECOND: f32 = 30.0;
        self.elapsed_time += time::get_frame_time();

        if self.elapsed_time >= 1.0 / BALLS_PER_SECOND {
            self.elapsed_time -= 1.0 / BALLS_PER_SECOND;

            for i in 0..self.spawner_count {
                self.balls.push(Ball {
                    pos: Vec2::new(2.0, 100.0 + 5.0 * i as f32),
                    vel: Vec2::new(400.0, 0.0),
                });
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

        if input::is_key_pressed(input::KeyCode::Left) && self.spawner_count != 0 {
            self.spawner_count -= 1;
        }
        if input::is_key_pressed(input::KeyCode::Right) {
            self.spawner_count += 1;
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            RECTANGLE.0 - RADIUS,
            RECTANGLE.1 - RADIUS,
            RECTANGLE.2 + RADIUS + RADIUS,
            RECTANGLE.3 + RADIUS + RADIUS,
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
