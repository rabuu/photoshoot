#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const BLACK: Rgb = Rgb {
    r: 0x00,
    g: 0x00,
    b: 0x00,
};
pub const WHITE: Rgb = Rgb {
    r: 0xff,
    g: 0xff,
    b: 0xff,
};
