use crate::object::ObjectSnapshot;

/// Dimension of the photos (used for width and height)
pub type PhotoDim = u16;

/// A single frame of the animation
pub struct Photo {
    width: PhotoDim,
    height: PhotoDim,
    objects: Vec<ObjectSnapshot>,
}
