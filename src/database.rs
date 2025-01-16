use macroquad::math::Vec2;

use crate::simulation::Simulation;

// const GRAPH_SIZE: usize = 300;
// const GAP: f32 = 3.0;
// const RECT_THICKNESS: f32 = 3.0;

pub struct Database {
    // kinetic_energy: [f32; GRAPH_SIZE],
    // potential_energy: [f32; GRAPH_SIZE],
    // mechanical_energy: [f32; GRAPH_SIZE],
    // frame_time: [f32; GRAPH_SIZE],
    // simulation_time: [f32; GRAPH_SIZE],
    // index: usize,

    // energy_enabed: bool,
    // frame_time_enabled: bool,
    // info_enabled: bool,
}

impl Database {
    pub fn new() -> Self {
        Self {
            // kinetic_energy: [0.0; GRAPH_SIZE],
            // potential_energy: [0.0; GRAPH_SIZE],
            // mechanical_energy: [0.0; GRAPH_SIZE],
            // frame_time: [0.0; GRAPH_SIZE],
            // simulation_time: [0.0; GRAPH_SIZE],
            // index: 0,

            // energy_enabed: true,
            // frame_time_enabled: true,
            // info_enabled: true,
        }
    }

    pub fn update(&mut self, simulation: &Simulation, simulation_time: f32) {}

    pub fn input(&mut self) {}

    pub fn draw(&self) {}
}
