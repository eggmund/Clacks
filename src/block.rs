use nannou::draw::Draw;
use nannou::color::named::*;

use crate::FLOOR_HEIGHT;

pub struct Block {
    pub pos: f64,       // One dimensional
    pub vel: f64,
    pub mass: f64,
    pub size: f32,
}

impl Block {
    pub fn new(pos: f64, vel: f64, mass: f64, size: f32) -> Block {
        Block {
            pos,
            vel,
            mass,
            size
        }
    }

    pub fn display(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.pos as f32, FLOOR_HEIGHT + self.size/2.0)
            .w_h(self.size, self.size)
            .color(RED);
    }

    #[inline]
    pub fn update_pos(&mut self, dt: f64) {
        self.pos += self.vel * dt;
    }

    pub fn bounce(&mut self, other: &Self) -> f64 {
        let total_mass = self.mass + other.mass;
        let mut new_vel = ((self.mass-other.mass)/total_mass) * self.vel;
        new_vel += (2.0 * other.mass / total_mass) * other.vel;
        new_vel
    }

    pub fn hit_wall(&mut self, half_screen_w: f32) -> bool {
        if self.pos - self.size as f64/2.0 <= -half_screen_w as f64 {
            self.vel = -self.vel;
            return true
        }
        return false
    }
}