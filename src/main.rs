use std::fs;

mod verlet;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let image = format!("{}/image.jpg", std::env!("CARGO_MANIFEST_DIR"));
//     let image = image::io::Reader::open(&image)?.decode()?;

//     let frame = gif::Frame::from_rgb(
//         image.width() as u16,
//         image.height() as u16,
//         image.as_bytes(),
//     );

//     let image2 = format!("{}/image2.jpg", std::env!("CARGO_MANIFEST_DIR"));
//     let image2 = image::io::Reader::open(&image2)?.decode()?;

//     let frame2 = gif::Frame::from_rgb(
//         image2.width() as u16,
//         image2.height() as u16,
//         image2.as_bytes(),
//     );

//     let mut gif = fs::File::create("gif.gif").unwrap();
//     let mut enc = gif::Encoder::new(&mut gif, frame.width, frame.height, &[]).unwrap();

//     enc.set_repeat(gif::Repeat::Infinite).unwrap();
//     enc.write_frame(&frame).unwrap();
//     enc.write_frame(&frame2).unwrap();

//     Ok(())
// }

// FIXME: The use of `nannou` is just for debugging purposes
use nannou::prelude::*;
fn main() {
    nannou::app(Model::new).update(update).run();
}

struct Model {
    _win: WindowId,
    solver: verlet::Solver,
}

impl Model {
    fn new(app: &App) -> Model {
        let _win = app
            .new_window()
            .size(2000, 2000)
            .title("PHOTOSHOOT")
            .view(view)
            .build()
            .unwrap();

        let solver = verlet::Solver {
            objects: vec![verlet::VerletObject::new_at_origin(20.0)],
            gravity: glam::Vec2::new(0.0, -2.0),
        };

        Model { _win, solver }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);
    draw.rect().width(800.0).height(750.0).color(BLACK);

    for obj in &model.solver.objects {
        draw.ellipse()
            .x_y(obj.pos.x, obj.pos.y)
            .radius(obj.radius)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last;
    model.solver.update(dt.as_secs_f32() * 10.);
}
