fn main() -> Result<(), Box<dyn std::error::Error>> {
    let black = std::cell::OnceCell::new();
    let _ = black.set(photoshoot::rgb::BLACK);

    let mut photos = Vec::new();

    for i in (0..300).step_by(10) {
        let photo: photoshoot::Photo = photoshoot::Photo::new(
            300,
            300,
            photoshoot::rgb::WHITE,
            vec![photoshoot::ObjectSnapshot {
                radius: 30.0,
                pos: glam::Vec2::new(i as f32, i as f32),
                color: std::rc::Rc::new(black.clone()),
            }],
        );

        photos.push(photo);
    }

    let photos_rev = photos.clone().into_iter().skip(1).rev();

    let _ = photos.pop();

    let mut gif = std::fs::File::create("gif.gif").unwrap();
    let mut enc = gif::Encoder::new(&mut gif, 300, 300, &[]).unwrap();

    enc.set_repeat(gif::Repeat::Infinite).unwrap();

    for photo in photos.into_iter().chain(photos_rev) {
        let frame = photo.into();
        enc.write_frame(&frame).unwrap();
    }

    Ok(())
}
