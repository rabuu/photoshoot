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

impl From<Photo> for gif::Frame<'_> {
    fn from(val: Photo) -> Self {
        let mut pixels: Vec<u8> = Vec::with_capacity(val.width as usize * val.height as usize);

        for y in 0..val.height {
            for x in 0..val.width {
                let mut px = val.background;

                for obj in &val.objects {
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

        gif::Frame::from_rgb_speed(val.width, val.height, &pixels, val.speed as i32)
    }
}
