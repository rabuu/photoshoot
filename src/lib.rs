use glam::Vec2;
use image::RgbImage;
use itertools::Itertools;

use object::Object;

// FIXME: public API
pub use gravity::Gravity;
pub use object::ObjectSnapshot;
pub use photo::Photo;
pub use rgb::Rgb;

mod gravity;
mod object;
mod photo;
pub mod rgb;

/// Main type that runs the photoshoot
pub struct Photoshoot {
    img: RgbImage,
    background: Rgb,
    frame_rate: f32,
    substeps: usize,
    gravity: Gravity,

    objects: Vec<Object>,
}

impl Photoshoot {
    pub fn new(
        img: RgbImage,
        background: Rgb,
        frame_rate: f32,
        substeps: usize,
        gravity: Gravity,
    ) -> Option<Self> {
        Some(Self {
            img,
            background,
            frame_rate,
            substeps,
            gravity,
            objects: Vec::new(),
        })
    }

    pub fn shoot(&mut self) -> Vec<Photo> {
        let radius = 4.0;
        let acc = Vec2::new(1.0, 2.0);

        let mut photos = Vec::with_capacity(100);
        for i in (0..1000).step_by(10) {
            let pos = Vec2::new(i as f32, self.img.height() as f32 / 2.0);
            self.objects.push(Object::new(radius, pos, acc));

            self.step();
            photos.push(self.photo());
        }

        // colorize objects
        for obj in &self.objects {
            let _ = obj.color.set(rgb::BLACK);
        }

        photos
    }

    pub fn photo(&self) -> Photo {
        Photo::new(
            self.img.width() as u16,
            self.img.height() as u16,
            self.background,
            self.objects.iter().map(Object::snapshot).collect(),
        )
    }

    fn step(&mut self) {
        // use substeps for better stability
        let sub_rate = self.frame_rate / self.substeps as f32;
        for _ in 0..self.substeps {
            // apply gravity
            for obj in &mut self.objects {
                obj.accelerate(self.gravity.into());
            }

            self.collisions();
            self.wall_constraint();

            // update each object
            for obj in &mut self.objects {
                obj.update_position(sub_rate);
            }
        }
    }

    /// Solve collisions of the objects
    ///
    /// Note that the used algorithm is very naive and *extremley* ineffienct and slow.
    /// Please FIXME if you want to, for example by space partitioning.
    fn collisions(&mut self) {
        for (obj1, obj2) in self.objects.iter().tuple_combinations() {
            // ugly fix fix when two objects spawn at the same position
            // it should be handled somewhere else this does not happen
            if *obj1.pos.borrow() == *obj2.pos.borrow() {
                *obj1.pos.borrow_mut() += 0.01;
                *obj2.pos.borrow_mut() -= 0.01;
            }

            let collision_axis = *obj1.pos.borrow() - *obj2.pos.borrow();
            let dist = collision_axis.length();

            if dist < obj1.radius + obj2.radius {
                let n = collision_axis / dist;
                let delta = (obj1.radius + obj2.radius) - dist;

                *obj1.pos.borrow_mut() += 0.5 * delta * n;
                *obj2.pos.borrow_mut() -= 0.5 * delta * n;
            }
        }
    }

    fn wall_constraint(&mut self) {
        let width = self.img.width() as u16;
        let height = self.img.height() as u16;

        for obj in &self.objects {
            let mut pos = obj.pos.borrow_mut();

            // (0, 0) ist top left

            // check left wall
            let wall_pos = 0.0;
            if (pos.x - obj.radius) < wall_pos {
                pos.x = wall_pos + obj.radius;
            }

            // check right wall
            let wall_pos = width as f32;
            if (pos.x + obj.radius) > wall_pos {
                pos.x = wall_pos - obj.radius;
            }

            // check top wall
            let wall_pos = 0.0;
            if (pos.y - obj.radius) < wall_pos {
                pos.y = wall_pos + obj.radius;
            }

            // check bottom wall
            let wall_pos = height as f32;
            if (pos.y + obj.radius) > wall_pos {
                pos.y = wall_pos - obj.radius;
            }
        }
    }
}
