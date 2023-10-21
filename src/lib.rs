use image::RgbImage;

use object::Object;

// FIXME: public API
pub use object::ObjectSnapshot;
pub use photo::Photo;
pub use rgb::Rgb;

mod object;
mod photo;
pub mod rgb;

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
