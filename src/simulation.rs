mod ball_obj;

pub use ball_obj::Ball;

use ball_obj::RADIUS;
use macroquad::{
    color,
    input::{self, is_mouse_button_pressed, mouse_position, MouseButton},
    math::Vec2,
    shapes::{draw_circle, draw_rectangle_lines},
    time,
};

const RECTANGLE: (f32, f32, f32, f32) = (25.0, 25.0, 650.0, 650.0);
const GRID_CELL_SIZE: f32 = 2.0 * RADIUS;

pub struct Simulation {
    pub balls: Vec<Ball>,
    pub spawner_count: usize,
    elapsed_time: f32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            balls: Vec::new(),
            spawner_count: 1,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        const SUB_STEPS: usize = 8;

        let dt = dt / SUB_STEPS as f32;

        for _ in 0..SUB_STEPS {
            let grid_rows = (RECTANGLE.3 / GRID_CELL_SIZE).ceil() as usize;
            let grid_cols = (RECTANGLE.2 / GRID_CELL_SIZE).ceil() as usize;
            let mut grid = vec![vec![vec![]; grid_cols]; grid_rows];

            for (i, ball) in self.balls.iter_mut().enumerate() {
                ball.update(dt);

                ball.move_in_bounds(RECTANGLE);

                grid[(ball.pos.x / GRID_CELL_SIZE) as usize]
                    [(ball.pos.y / GRID_CELL_SIZE) as usize]
                    .push(i);
            }

            for i in 0..(grid_rows - 1) {
                for j in 0..(grid_cols - 1) {
                    let mut ball_ids: Vec<usize> = Vec::new();

                    ball_ids.extend(&grid[i][j]);
                    ball_ids.extend(&grid[i][j + 1]);
                    ball_ids.extend(&grid[i + 1][j]);
                    ball_ids.extend(&grid[i + 1][j + 1]);

                    Simulation::check_each_combo(&mut self.balls, &ball_ids);
                }
            }
        }
    }

    fn check_each_combo(balls: &mut Vec<Ball>, ball_ids: &Vec<usize>) {
        for a in 0..ball_ids.len() {
            for b in (a + 1)..ball_ids.len() {
                let (a, b) = if ball_ids[a] < ball_ids[b] {
                    (ball_ids[a], ball_ids[b])
                } else {
                    (ball_ids[b], ball_ids[a])
                };
                let (left, right) = balls.split_at_mut(b);
                Simulation::solve_collision(&mut left[a], &mut right[0]);
            }
        }
    }

    fn solve_collision(a: &mut Ball, b: &mut Ball) {
        let impact = a.pos - b.pos;

        let len_sqr = impact.length_squared();
        if len_sqr < (RADIUS + RADIUS) * (RADIUS + RADIUS) {
            let dir = impact.normalize();

            let len = len_sqr.sqrt();
            let overlap = RADIUS + RADIUS - len;
            let corr = dir * overlap / 2.0;
            a.pos += corr;
            b.pos -= corr;

            let comp = Vec2::dot(dir, b.vel);
            if comp > 0.0 {
                b.vel -= dir * comp;
            }

            let dir = dir * -1.0;
            let comp = Vec2::dot(dir, a.vel);
            if comp > 0.0 {
                a.vel -= dir * comp;
            }
        }
    }

    pub fn spawn(&mut self) {
        const BALLS_PER_SECOND: f32 = 30.0;
        self.elapsed_time += time::get_frame_time();

        if self.elapsed_time >= 1.0 / BALLS_PER_SECOND {
            self.elapsed_time -= 1.0 / BALLS_PER_SECOND;

            if time::get_frame_time() < 0.012 {
                for i in 0..self.spawner_count {
                    let pos = Vec2::new(RADIUS, 100.0 + (3.0 * RADIUS) * i as f32);
                    self.balls.push(Ball {
                        pos,
                        vel: Vec2::new(500.0, 0.0),
                    });
                }
            }
        }
    }

    pub fn input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = Vec2::new(
                mouse_position().0 - RECTANGLE.0,
                mouse_position().1 - RECTANGLE.1,
            );
            self.balls.push(Ball {
                pos,
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
