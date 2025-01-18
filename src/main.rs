mod database;
mod simulation;

use std::time::Instant;

use database::Database;
use macroquad::{color, input, time, window};
use simulation::Simulation;

fn window_conf() -> window::Conf {
    const WINDOW_SIZE: (i32, i32) = (1006, 700);
    window::Conf {
        window_title: "Collision Sim".to_owned(),
        window_resizable: false,

        // window_width: WINDOW_SIZE.0,
        // window_height: WINDOW_SIZE.1,
        // high_dpi: false,
        window_width: 2 * WINDOW_SIZE.0,
        window_height: 2 * WINDOW_SIZE.1,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();
    let mut database = Database::new();

    loop {
        window::clear_background(color::BLACK);

        simulation.draw();
        database.draw();

        simulation.input();
        database.input();

        simulation.spawn();

        let start = Instant::now();
        simulation.update(time::get_frame_time());
        let simulation_time = (Instant::now() - start).as_secs_f32();

        database.update(&simulation, simulation_time);

        if input::is_key_down(input::KeyCode::Escape) {
            break;
        }

        window::next_frame().await
    }
}
