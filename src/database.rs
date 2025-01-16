use macroquad::{
    color,
    math::Vec2,
    shapes::{draw_line, draw_rectangle_lines},
    text::draw_text,
};

use crate::simulation::Simulation;

const GRAPH_SIZE: usize = 300;
const GAP: f32 = 3.0;
const RECT_THICKNESS: f32 = 3.0;

pub struct Database {
    kinetic_energy: [f32; GRAPH_SIZE],
    potential_energy: [f32; GRAPH_SIZE],
    mechanical_energy: [f32; GRAPH_SIZE],
    // frame_time: [f32; GRAPH_SIZE],
    // simulation_time: [f32; GRAPH_SIZE],
    index: usize,
    // energy_enabed: bool,
    // frame_time_enabled: bool,
    // info_enabled: bool,
}

impl Database {
    pub fn new() -> Self {
        Self {
            kinetic_energy: [0.0; GRAPH_SIZE],
            potential_energy: [0.0; GRAPH_SIZE],
            mechanical_energy: [0.0; GRAPH_SIZE],
            // frame_time: [0.0; GRAPH_SIZE],
            // simulation_time: [0.0; GRAPH_SIZE],
            index: 0,
            // energy_enabed: true,
            // frame_time_enabled: true,
            // info_enabled: true,
        }
    }

    pub fn update(&mut self, simulation: &Simulation, simulation_time: f32) {
        self.kinetic_energy[self.index] = simulation
            .balls
            .iter()
            .map(|ball| ball.vel)
            .fold(0.0, |acc, v| acc + (v.length() / 100.0).powi(2))
            / 2.0;

        self.potential_energy[self.index] = simulation
            .balls
            .iter()
            .map(|ball| ball.pos)
            .fold(0.0, |acc, p| acc + (450.0 - p.y) / 100.0 * 9.8);

        self.mechanical_energy[self.index] =
            self.kinetic_energy[self.index] + self.potential_energy[self.index];

        self.index += 1;
        if self.index >= GRAPH_SIZE {
            self.index = 0;
        }
    }

    pub fn input(&mut self) {}

    pub fn draw(&self) {
        let curr_index = if self.index == 0 {
            GRAPH_SIZE - 1
        } else {
            self.index - 1
        };

        const ID: i32 = 0;
        const TITLE_RECT: (f32, f32, f32, f32) = (
            500.0 + GAP,
            125.0 * ID as f32 + GAP * (2 * ID + 1) as f32,
            GRAPH_SIZE as f32,
            25.0,
        );
        draw_rectangle_lines(
            TITLE_RECT.0,
            TITLE_RECT.1,
            TITLE_RECT.2,
            TITLE_RECT.3,
            RECT_THICKNESS,
            color::PURPLE,
        );
        draw_text(
            &format!("Energy - {}", self.mechanical_energy[curr_index]),
            TITLE_RECT.0 + GAP,
            TITLE_RECT.1 + 17.0,
            23.0,
            color::LIGHTGRAY,
        );

        const RECT: (f32, f32, f32, f32) = (
            500.0 + GAP,
            125.0 * ID as f32 + GAP * (2 * ID + 2) as f32 + 25.0,
            GRAPH_SIZE as f32,
            100.0,
        );
        draw_rectangle_lines(
            RECT.0,
            RECT.1,
            RECT.2,
            RECT.3,
            RECT_THICKNESS,
            color::PURPLE,
        );

        let energy_scale = 75.0 / self.mechanical_energy[curr_index];

        for i in 0..(GRAPH_SIZE - 1) {
            if i + 1 == self.index {
                continue;
            }

            draw_line(
                RECT.0 + i as f32,
                RECT.1 + RECT.3 - self.kinetic_energy[i] * energy_scale,
                RECT.0 + (i + 1) as f32,
                RECT.1 + RECT.3 - self.kinetic_energy[i + 1] * energy_scale,
                1.0,
                color::RED,
            );
            draw_line(
                RECT.0 + i as f32,
                RECT.1 + RECT.3 - self.potential_energy[i] * energy_scale,
                RECT.0 + (i + 1) as f32,
                RECT.1 + RECT.3 - self.potential_energy[i + 1] * energy_scale,
                1.0,
                color::BLUE,
            );
            draw_line(
                RECT.0 + i as f32,
                RECT.1 + RECT.3 - self.mechanical_energy[i] * energy_scale,
                RECT.0 + (i + 1) as f32,
                RECT.1 + RECT.3 - self.mechanical_energy[i + 1] * energy_scale,
                1.0,
                color::PURPLE,
            );
        }
    }
}
