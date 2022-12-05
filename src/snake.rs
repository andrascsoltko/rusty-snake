use std::time::Duration;

use rand::{prelude::Distribution, distributions::Standard};
use rusty_time::timer::Timer;

use crate::{NUM_COLS, frame::{Drawable, Frame}, NUM_ROWS, pickup::Pickup};


pub struct Snake {
    body : Vec<Point>,
    move_timer: Timer,
    pub direction: Direction,
    pub dead: bool,
}

impl Snake {
    pub fn new() -> Self {
        let mut _body = Vec::new();
        for c in 0..6 {
            _body.push(Point::new(NUM_COLS/2 + c, NUM_COLS/2));
        }
        Self {
            body: _body,
            move_timer: Timer::from_millis(200),
            direction: rand::random(),
            dead: false,
        }
    }
    pub fn move_right(&mut self) {
        if self.body[0].x < NUM_COLS - 1 {
            let new_head = Point::new(self.body[0].x + 1, self.body[0].y);
            self.validate_move(new_head);
        }
    }
    pub fn move_left(&mut self) {
        if self.body[0].x > 0 {
            let new_head = Point::new(self.body[0].x - 1, self.body[0].y);
            self.validate_move(new_head);
        }
    }
    pub fn move_up(&mut self) {
        if self.body[0].y > 0 {
            let new_head = Point::new(self.body[0].x, self.body[0].y - 1);
            self.validate_move(new_head);
        }
    }
    pub fn move_down(&mut self) {
        if self.body[0].y < NUM_ROWS - 1 {
            let new_head = Point::new(self.body[0].x, self.body[0].y + 1);
            self.validate_move(new_head);
        }
    }
    pub fn eat_pickup(&mut self, pickup: &mut Pickup) -> bool {
        if self.body[0].x == pickup.x && self.body[0].y == pickup.y  {
            pickup.respawn();
            true
        }
        else{
            false
        }
    }
    pub fn grow(&mut self) {
        let last_item_index = self.body.len() - 1;
        let last_body_party = self.body[last_item_index].clone();
        self.body.insert(last_item_index,last_body_party)
    }
    pub fn change_direction(&mut self, direction: Direction) {
        
        if direction != self.direction {
            
            if direction == Direction::Right {
                if self.direction == Direction::Left{
                    return
                }
                self.direction = direction;
                return
            }
            if direction == Direction::Left {
                if self.direction == Direction::Right{
                    return
                }
                self.direction = direction;
                return
            }
            if direction == Direction::Up {
                if self.direction == Direction::Down{
                    return
                }
                self.direction = direction;
                return
            }
            if direction == Direction::Down {
                if self.direction == Direction::Up{
                    return
                }
                self.direction = direction;
                return
            }

        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            match self.direction {
                Direction::Left => self.move_left(),
                Direction::Right => self.move_right(),
                Direction::Up => self.move_up(),
                Direction::Down => self.move_down(),
            }
            return true;
        }
        false
    }
    // Allows the player to follow its tail!
    fn validate_move(&mut self, new_head: Point) {
        let mut body_copy = self.body.to_vec();
            body_copy.pop();
            if !body_copy.contains(&new_head) {
                self.body.insert(0, new_head);
                self.body.pop();
            } 
            else {
                self.dead = true;
            }
    }
}

impl Drawable for Snake {
    fn draw(&self, frame: &mut Frame) {
        for part in self.body.iter() {
            frame[part.x][part.y] = "¤";
        }
        frame[self.body[0].x][self.body[0].y] = "#";
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=2) {
            0 => Direction::Up,
            1 => Direction::Down,
            _ => Direction::Left,
        }
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}