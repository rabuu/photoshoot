use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

use glam::Vec2;

use crate::rgb::Rgb;

pub struct Object {
    radius: f32,

    pos: RefCell<Vec2>,
    pos_prev: Vec2,
    acc: Vec2,

    color: Rc<OnceCell<Rgb>>,
}

impl Object {
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
