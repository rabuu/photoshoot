use crate::object::ObjectSnapshot;
use crate::rgb::Rgb;

/// A single frame of the animation
#[derive(Debug, Clone)]
pub struct Photo {
    width: u16,
    height: u16,
    background: Rgb,
    /// `speed` is in the range [1, 30]
    speed: u8,
    objects: Vec<ObjectSnapshot>,
}

impl Photo {
    pub fn new(
        width: u16,
        height: u16,
        background: Rgb,
        speed: u8,
        objects: Vec<ObjectSnapshot>,
    ) -> Self {
        let speed = speed.clamp(1, 30);
        Self {
            width,
            height,
            background,
            speed,
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
                    if obj.intersects(x as f32, y as f32) {
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

        gif::Frame::from_rgb_speed(self.width, self.height, &pixels, self.speed as i32)
    }
}
