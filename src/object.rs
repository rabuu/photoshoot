use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

use glam::Vec2;

use crate::rgb::Rgb;

pub struct Object {
    pub radius: f32,

    pub pos: RefCell<Vec2>,
    pos_prev: Vec2,
    acc: Vec2,

    pub color: Rc<OnceCell<Rgb>>,
}

impl Object {
    pub fn new(radius: f32, pos: Vec2, pos_prev: Vec2) -> Self {
        Self {
            radius,
            pos: RefCell::new(pos),
            pos_prev,
            acc: Vec2::ZERO,
            color: Rc::new(OnceCell::new()),
        }
    }

    pub fn update_position(&mut self, rate: f32) {
        let vel = *self.pos.borrow() - self.pos_prev;

        // save current position
        self.pos_prev = *self.pos.borrow();

        // perform Verlet integration
        *self.pos.borrow_mut() = self.pos_prev + vel + self.acc * rate * rate;

        // reset acceleration
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }

    pub fn snapshot(&self) -> ObjectSnapshot {
        ObjectSnapshot {
            radius: self.radius,
            pos: *self.pos.borrow(),
            color: Rc::clone(&self.color),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectSnapshot {
    pub radius: f32,
    pub pos: Vec2,
    pub color: Rc<OnceCell<Rgb>>,
}

impl ObjectSnapshot {
    #[cfg(debug_assertions)]
    pub fn instant(radius: f32, x: f32, y: f32, col: Rgb) -> Self {
        let color = OnceCell::new();
        let _ = color.set(col);

        Self {
            radius,
            pos: Vec2::new(x, y),
            color: Rc::new(color),
        }
    }
}
