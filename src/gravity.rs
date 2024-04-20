use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Gravity(f32);

impl Gravity {
    pub fn new(g: f32) -> Self {
        Self(g * 100.0)
    }
}

impl Into<Vec2> for Gravity {
    fn into(self) -> Vec2 {
        Vec2::new(0.0, self.0)
    }
}
