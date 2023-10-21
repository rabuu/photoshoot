use glam::Vec2;

use crate::object::Object;

pub struct Canon {
    radius: f32,
    pub pos: Vec2,
    dir: Vec2,
    rotation: Option<Vec2>,
}

impl Canon {
    pub fn new(radius: f32, pos: Vec2, dir: Vec2, rotation: Option<Vec2>) -> Self {
        Self {
            radius,
            pos,
            dir,
            rotation,
        }
    }

    pub fn shoot(&mut self, objects: &[Object]) -> Option<Object> {
        if self.is_covered(objects) {
            return None;
        }

        let obj = Some(Object::new(self.radius, self.pos, self.pos - self.dir));

        if let Some(rot) = self.rotation {
            self.dir = rot.normalize().rotate(self.dir);
        }

        obj
    }

    pub fn is_covered(&self, objects: &[Object]) -> bool {
        for obj in objects {
            if (*obj.pos.borrow() - self.pos).length() <= obj.radius + self.radius {
                return true;
            }
        }

        false
    }
}
