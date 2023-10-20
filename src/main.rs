use photoshoot::physics;

// use std::fs;
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
    solver: physics::Solver,
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

        let solver = physics::Solver::new(
            vec![
                physics::Object::new_at_origin(20.0),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 30.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 40.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 50.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 60.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 70.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 80.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 90.0)),
                physics::Object::new(5.0, glam::Vec2::new(0.0, 100.0)),
            ],
            glam::Vec2::new(0.0, -98.1),
        );

        Model { _win, solver }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);
    draw.rect().width(800.0).height(750.0).color(BLACK);

    for obj in &model.solver.objects {
        let obj = obj.borrow();

        draw.ellipse()
            .x_y(obj.pos.x, obj.pos.y)
            .radius(obj.radius)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last;
    model.solver.update(dt.as_secs_f32());
}
