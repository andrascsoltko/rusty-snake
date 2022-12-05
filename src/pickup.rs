use rand::Rng;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, self}};

pub struct Pickup {
    pub x: usize,
    pub y: usize,
}

impl Pickup {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x : rng.gen_range(0..NUM_COLS),
            y : rng.gen_range(0..NUM_ROWS),
        }
    }
    pub fn respawn(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0..NUM_COLS);
        self.y = rng.gen_range(0..NUM_ROWS);
    }
}

impl Drawable for Pickup {
    fn draw(&self, frame: &mut frame::Frame) {
        frame[self.x][self.y] = "A"
    }
}
