use glam::Vec2;
use image::RgbImage;
use itertools::Itertools;

use canon::{Canon, CanonMode};
use object::Object;

// FIXME: public API
pub use gravity::Gravity;
pub use object::ObjectSnapshot;
pub use photo::Photo;
pub use rgb::Rgb;

mod canon;
mod gravity;
mod object;
mod photo;
pub mod rgb;

/// Main type that runs the photoshoot
pub struct Photoshoot {
    img: RgbImage,
    background: Rgb,
    frame_rate: f32,
    gif_speed: u8,
    substeps: usize,
    gravity: Gravity,

    objects: Vec<Object>,
    canon: Canon,
}

impl Photoshoot {
    pub fn new(
        img: RgbImage,
        background: Rgb,
        frame_rate: f32,
        gif_speed: u8,
        substeps: usize,
        gravity: Gravity,
        radius: f32,
    ) -> Option<Self> {
        let center = Vec2::new(img.width() as f32 / 2.0, img.height() as f32 / 2.0);

        Some(Self {
            img,
            background,
            frame_rate,
            gif_speed,
            substeps,
            gravity,
            objects: Vec::new(),
            canon: Canon::new(radius, center, CanonMode::default()),
        })
    }

    pub fn run(&mut self) -> Vec<Photo> {
        eprintln!("Start photoshoot...");

        let mut photos = Vec::new();

        const ALLOWED_MISSES: usize = 10;
        let mut missed = 0;

        for i in 0.. {
            eprint!("\rClick! [{}]", i + 1);

            let (full, maybe_object) = self.canon.shoot(&self.objects);

            if full {
                break;
            }

            if let Some(obj) = maybe_object {
                self.objects.push(obj);
                missed = 0;
            } else {
                missed += 1;
            }

            if missed >= ALLOWED_MISSES {
                self.canon.mode = CanonMode::Fill {
                    top: 0,
                    bottom: self.img.height() as usize,
                    left: 0,
                    right: self.img.width() as usize,
                };
            }

            self.step();
            photos.push(self.photo());
        }
        eprintln!();

        // colorize objects
        eprintln!("Colorizing...");
        for obj in &self.objects {
            let mut count: usize = 0;
            let mut r: f32 = 0.0;
            let mut g: f32 = 0.0;
            let mut b: f32 = 0.0;

            for (i, px) in self.img.pixels().enumerate() {
                let x = i % self.img.width() as usize;
                let y = i / self.img.width() as usize;

                if obj.intersects(x as f32, y as f32) {
                    r += px.0[0] as f32 * px.0[0] as f32;
                    g += px.0[1] as f32 * px.0[1] as f32;
                    b += px.0[2] as f32 * px.0[2] as f32;
                    count += 1;
                }
            }

            if count > 0 {
                let col = Rgb {
                    r: (r / count as f32).sqrt() as u8,
                    g: (g / count as f32).sqrt() as u8,
                    b: (b / count as f32).sqrt() as u8,
                };

                obj.color.set(col).expect("Color is set just once");
            }
        }

        photos
    }

    pub fn photo(&self) -> Photo {
        Photo::new(
            self.img.width() as u16,
            self.img.height() as u16,
            self.background,
            self.gif_speed,
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
