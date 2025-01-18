use macroquad::math::Vec2;

#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn update(&mut self, dt: f32) {
        const GRAVITY: Vec2 = Vec2::new(0.0, 980.0);

        const VELOCITY_DAMPING: f32 = 40.0;
        let last_updated_move = self.pos - self.prev_pos;
        let new_pos = self.pos
            + last_updated_move
            + (GRAVITY - last_updated_move * VELOCITY_DAMPING) * (dt * dt);

        if (new_pos - self.pos).length() > 5.0 {
            self.prev_pos = self.pos;
            return;
        }

        self.prev_pos = self.pos;
        self.pos = new_pos;
    }

    pub fn move_in_bounds(&mut self, rectangle: (f32, f32, f32, f32)) {
        if self.pos.y >= rectangle.3 - self.radius {
            self.pos.y = 2.0 * (rectangle.3 - self.radius) - self.pos.y;
        }
        if self.pos.y <= self.radius {
            self.pos.y = 2.0 * self.radius - self.pos.y;
        }

        if self.pos.x >= rectangle.2 - self.radius {
            self.pos.x = 2.0 * (rectangle.2 - self.radius) - self.pos.x;
        }
        if self.pos.x <= self.radius {
            self.pos.x = 2.0 * self.radius - self.pos.x;
        }
    }
}
