use glam::Vec2;

use crate::object::ObjectSnapshot;
use crate::rgb::Rgb;

/// A single frame of the animation
#[derive(Debug, Clone)]
pub struct Photo {
    width: u16,
    height: u16,
    background: Rgb,
    objects: Vec<ObjectSnapshot>,
}

impl Photo {
    pub fn new(width: u16, height: u16, background: Rgb, objects: Vec<ObjectSnapshot>) -> Self {
        Self {
            width,
            height,
            background,
            objects,
        }
    }
}

impl Into<gif::Frame<'_>> for Photo {
    fn into(self) -> gif::Frame<'static> {
        let mut pixels: Vec<u8> = Vec::with_capacity(self.width as usize * self.height as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut px = self.background;

                for obj in &self.objects {
                    let dist_squared = (obj.pos - Vec2::new(x as f32, y as f32)).length_squared();
                    if dist_squared <= obj.radius * obj.radius {
                        if let Some(&col) = obj.color.get() {
                            px = col;
                        }
                    }
                }

                pixels.push(px.r);
                pixels.push(px.g);
                pixels.push(px.b);
            }
        }

        gif::Frame::from_rgb(self.width, self.height, &pixels)
    }
}
