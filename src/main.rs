mod database;
mod simulation;

use std::time::Instant;

use database::Database;
use macroquad::{color, input, math::Vec2, time, window};
use simulation::{Ball, Simulation};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Collision Sim".to_owned(),
        window_resizable: false,
        window_width: 806,
        window_height: 500,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::new();
    let mut database = Database::new();

    let mut spawn_counter = 0;
    loop {
        window::clear_background(color::BLACK);

        simulation.draw();
        database.draw();

        simulation.input();
        database.input();

        if time::get_frame_time() < 0.017 {
            spawn_counter += 1;
            if spawn_counter == 3 {
                spawn_counter = 0;

                simulation.balls.push(Ball {
                    pos: Vec2::new(2.0, 100.0),
                    vel: Vec2::new(400.0, 0.0),
                });
            }
        }

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
