fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(&format!("{}/image.jpg", env!("CARGO_MANIFEST_DIR")))?.into_rgb8();

    // TODO: bounds check
    let width = img.width() as u16;
    let height = img.height() as u16;

    let mut photoshoot = photoshoot::Photoshoot::new(
        img,
        photoshoot::rgb::WHITE,
        1.0 / 60.0,
        30,
        8,
        photoshoot::Gravity::new(900.81),
        10.0,
    )
    .unwrap();

    let photos = photoshoot.run();

    let mut gif = std::fs::File::create("gif.gif").unwrap();
    let mut enc = gif::Encoder::new(&mut gif, width, height, &[]).unwrap();

    enc.set_repeat(gif::Repeat::Finite(1)).unwrap();

    let count = photos.len();
    for (i, photo) in photos.into_iter().enumerate() {
        let frame = photo.into();
        enc.write_frame(&frame).unwrap();
        eprintln!("Wrote frame {}/{count}.", i + 1);
    }

    Ok(())
}
