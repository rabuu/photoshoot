use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the input photo
    photo: PathBuf,

    /// Path to the output GIF
    #[arg(short, long)]
    output: PathBuf,

    /// Frame rate of the GIF
    #[arg(short, long, default_value_t = 0.02)]
    frame_rate: f32,

    /// Speed of the writing of the GIF
    #[arg(long, default_value_t = 10)]
    gif_speed: u8,

    /// Simulation substeps
    #[arg(short, long, default_value_t = 5)]
    substeps: usize,

    /// Simulation gravity
    #[arg(long, default_value_t = 9.81)]
    gravity: f32,

    /// Radius of the canon balls
    #[arg(short, long, default_value_t = 1.0)]
    radius: f32,

    /// Black background (default: white)
    #[arg(short, long)]
    black_bg: bool,

    /// Should the GIF run repeat infinitely
    #[arg(short, long)]
    infinite: bool,

    /// How much longer should the last frame be
    #[arg(short, long, default_value_t = 50)]
    last_frame: usize,

    /// Don't write to file
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let photo = image::open(cli.photo)?.into_rgb8();

    let width: u16 = photo.width().try_into()?;
    let height: u16 = photo.height().try_into()?;

    let bg = if cli.black_bg {
        photoshoot::rgb::BLACK
    } else {
        photoshoot::rgb::WHITE
    };

    let mut photoshoot = photoshoot::Photoshoot::new(
        photo,
        bg,
        cli.frame_rate,
        cli.gif_speed,
        cli.substeps,
        photoshoot::Gravity::new(cli.gravity),
        cli.radius,
    )
    .unwrap();

    let photos = photoshoot.run();
    let last_photo = photos.last().unwrap().clone();

    if cli.dry_run {
        eprintln!("Exit because of dry run...");
        return Ok(());
    }

    eprintln!("Creating file {:?}...", cli.output);
    let mut gif = std::fs::File::create_new(cli.output)?;
    let mut enc = gif::Encoder::new(&mut gif, width, height, &[])?;

    let repeat = if cli.infinite {
        gif::Repeat::Infinite
    } else {
        gif::Repeat::Finite(1)
    };

    enc.set_repeat(repeat).unwrap();

    let count = photos.len();
    for (i, photo) in photos.into_iter().enumerate() {
        let frame = photo.into();
        enc.write_frame(&frame)?;
        eprint!("\rWrote frame {}/{count}.", i + 1);
    }
    eprintln!();

    let last_frame = last_photo.into();
    eprintln!("Last frame...");
    for _ in 0..cli.last_frame {
        enc.write_frame(&last_frame).unwrap();
    }

    Ok(())
}
