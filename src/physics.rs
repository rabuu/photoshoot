use std::cell::RefCell;

use glam::Vec2;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Object {
    pub pos: Vec2,
    prev_pos: Vec2,
    acc: Vec2,

    pub radius: f32,
}

impl Object {
    pub fn new(radius: f32, pos: Vec2) -> Self {
        Self {
            pos,
            prev_pos: pos,
            acc: Vec2::ZERO,
            radius,
        }
    }

    pub fn new_at_origin(radius: f32) -> Self {
        Self::new(radius, Vec2::ZERO)
    }

    pub fn update_position(&mut self, dt: f32) {
        let vel = self.pos - self.prev_pos;

        // save current position
        self.prev_pos = self.pos;

        // perform Verlet integration
        self.pos = self.pos + vel + self.acc * dt * dt;

        // reset acceleration
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }
}

pub struct Solver {
    // help, i am a RefCell, please FIXME
    pub objects: Vec<RefCell<Object>>,
    pub gravity: Vec2,
}

impl Solver {
    pub fn new(objects: Vec<Object>, gravity: Vec2) -> Self {
        Self {
            objects: objects.into_iter().map(RefCell::new).collect(),
            gravity,
        }
    }

    pub fn update(&mut self, dt: f32) {
        const SUBSTEPS: usize = 10;
        let sub_dt = dt / SUBSTEPS as f32;

        for _ in 0..SUBSTEPS {
            // apply gravity
            for obj in &self.objects {
                obj.borrow_mut().accelerate(self.gravity);
            }

            self.collisions();
            self.apply_wall_constraint();

            // update positions
            for obj in &self.objects {
                obj.borrow_mut().update_position(sub_dt);
            }
        }
    }

    fn apply_wall_constraint(&mut self) {
        let width: u16 = 800;
        let height: u16 = 750;

        for obj in &self.objects {
            let mut obj = obj.borrow_mut();

            // check left wall
            let wall_pos = -(width as f32) / 2.0;
            if (obj.pos.x - obj.radius) < wall_pos {
                obj.pos.x = wall_pos + obj.radius;
            }

            // check right wall
            let wall_pos = (width as f32) / 2.0;
            if (obj.pos.x + obj.radius) > wall_pos {
                obj.pos.x = wall_pos - obj.radius;
            }

            // check bottom wall
            let wall_pos = -(height as f32) / 2.0;
            if (obj.pos.y - obj.radius) < wall_pos {
                obj.pos.y = wall_pos + obj.radius;
            }

            // check top wall
            let wall_pos = (height as f32) / 2.0;
            if (obj.pos.y + obj.radius) > wall_pos {
                obj.pos.y = wall_pos - obj.radius;
            }
        }
    }

    /// Solve collisions of the objects
    ///
    /// Note that the used algorithm is very naive and *extremley* ineffienct and slow.
    /// Please FIXME if you want to, for example by space partitioning.
    fn collisions(&mut self) {
        for (obj1, obj2) in self.objects.iter().tuple_combinations() {
            let mut obj1 = obj1.borrow_mut();
            let mut obj2 = obj2.borrow_mut();

            // ugly fix fix when two objects spawn at the same position
            // it should be handled somewhere else this does not happen
            if obj1.pos == obj2.pos {
                obj1.pos += 0.01;
                obj2.pos -= 0.01;
            }

            let collision_axis = obj1.pos - obj2.pos;
            let dist = collision_axis.length();

            if dist < obj1.radius + obj2.radius {
                let n = collision_axis / dist;
                let delta = (obj1.radius + obj2.radius) - dist;

                obj1.pos += 0.5 * delta * n;
                obj2.pos -= 0.5 * delta * n;
            }
        }
    }
}
