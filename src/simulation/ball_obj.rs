use macroquad::math::Vec2;

pub const RADIUS: f32 = 2.5;

#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Ball {
    pub fn update(&mut self, dt: f32) {
        const GRAVITY: Vec2 = Vec2::new(0.0, 980.0);
        const VELOCITY_DAMPING: f32 = 1.0;

        self.vel *= VELOCITY_DAMPING;
        self.vel += GRAVITY * dt;

        self.pos += self.vel * dt;
    }

    pub fn move_in_bounds(&mut self, rectangle: (f32, f32, f32, f32)) {
        if self.pos.y >= rectangle.3 - RADIUS {
            self.pos.y = 2.0 * (rectangle.3 - RADIUS) - self.pos.y;
            self.vel.y *= -1.0;
        }
        if self.pos.y <= RADIUS {
            self.pos.y = 2.0 * RADIUS - self.pos.y;
            self.vel.y *= -1.0;
        }

        if self.pos.x >= rectangle.2 - RADIUS {
            self.pos.x = 2.0 * (rectangle.2 - RADIUS) - self.pos.x;
            self.vel.x *= -1.0;
        }
        if self.pos.x <= RADIUS {
            self.pos.x = 2.0 * RADIUS - self.pos.x;
            self.vel.x *= -1.0;
        }
    }
}
