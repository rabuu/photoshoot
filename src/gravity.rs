use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Gravity(f32);

impl Gravity {
    pub fn new(g: f32) -> Self {
        Self(g * 100.0)
    }
}

impl From<Gravity> for Vec2 {
    fn from(val: Gravity) -> Self {
        Vec2::new(0.0, val.0)
    }
}
