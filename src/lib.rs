use image::RgbImage;

use object::Object;
use photo::Photo;

mod object;
mod photo;
mod rgb;

/// Main type that runs the photoshoot
pub struct Photoshoot {
    img: RgbImage,
    objects: Vec<Object>,
    frame_rate: f32,
}

impl Photoshoot {
    pub fn shoot(&mut self) -> Vec<Photo> {
        todo!()
    }
}
