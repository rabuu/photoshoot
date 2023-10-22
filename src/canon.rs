use glam::Vec2;

use crate::object::Object;

pub struct Canon {
    radius: f32,
    pub pos: Vec2,
    pub mode: CanonMode,
}

impl Canon {
    pub fn new(radius: f32, pos: Vec2, mode: CanonMode) -> Self {
        Self { radius, pos, mode }
    }

    pub fn shoot(&mut self, objects: &[Object]) -> (bool, Option<Object>) {
        let obj = if self.is_covered(objects) {
            None
        } else {
            let pos_prev = if let CanonMode::Rotation { dir, rot: _ } = self.mode {
                self.pos - dir
            } else {
                self.pos
            };

            Some(Object::new(self.radius, self.pos, pos_prev))
        };

        match self.mode {
            CanonMode::Rotation { dir: old_dir, rot } => {
                self.mode = CanonMode::Rotation {
                    dir: rot.normalize().rotate(old_dir),
                    rot,
                };
            }
            CanonMode::Fill {
                top,
                bottom,
                left,
                right,
            } => {
                let mut full = true;
                'outer: for y in (top..=bottom).step_by(self.radius as usize) {
                    for x in (left..=right).step_by(self.radius as usize) {
                        self.pos = Vec2::new(x as f32, y as f32);
                        if !self.is_covered(objects) {
                            full = false;
                            break 'outer;
                        }
                    }
                }

                if full {
                    return (true, None);
                } else {
                    self.mode = CanonMode::default()
                }
            }
        }

        (false, obj)
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

pub enum CanonMode {
    Rotation {
        dir: Vec2,
        rot: Vec2,
    },
    Fill {
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
    },
}

impl Default for CanonMode {
    fn default() -> Self {
        CanonMode::Rotation {
            dir: Vec2::new(3.0, 0.0),
            rot: Vec2::new(1.0, 0.1),
        }
    }
}
