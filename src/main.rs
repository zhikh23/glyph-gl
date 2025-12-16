pub mod app;
pub mod camera;
pub mod config;
pub mod geometry;
pub mod io;
pub mod math;
pub mod output;
pub mod rendering;

use clap::{Arg, ArgAction, Command, value_parser};
use crossterm::terminal;
use std::error::Error;

use crate::app::App;
use crate::config::{Config, ShadingMode};
use crate::math::vectors::Vector3;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = build_cli().get_matches();

    let terminal_size = terminal::size().unwrap_or((80, 24));
    let config = Config::default()
        .with_resolution(2 * terminal_size.0 as usize, 4 * terminal_size.1 as usize)
        .with_clap_matches(&matches);

    let input_path = matches.get_one::<String>("model").unwrap();
    let mut app = App::new(input_path, config);
    app.run()
}

fn build_cli() -> Command {
    Command::new("GlyphGL")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Kirill Zhikharev")
        .about("Subpixel terminal 3D .obj render")
        .arg(
            Arg::new("model")
                .required(true)
                .index(1)
                .help("Path to Wavefront OBJ (.obj) model file"),
        )
        .arg(
            Arg::new("static-mode")
                .long("static")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Static mode: render single frame"),
        )
        .arg(
            Arg::new("frame-width")
                .short('W')
                .long("width")
                .value_parser(value_parser!(usize))
                .help("Width of output image"),
        )
        .arg(
            Arg::new("frame-height")
                .short('H')
                .long("height")
                .value_parser(value_parser!(usize))
                .help("Height of output image"),
        )
        .arg(
            Arg::new("no-culling")
                .long("no-culling")
                .help("Disable backface culling")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("shading")
                .long("shading")
                .value_parser(value_parser!(ShadingMode))
                .help("Shading mode"),
        )
        .arg(
            Arg::new("camera-speed")
                .long("camera-speed")
                .value_parser(value_parser!(f32))
                .help("Camera speed"),
        )
        .arg(
            Arg::new("camera-rotation-speed")
                .long("camera-rotation-speed")
                .value_parser(value_parser!(f32))
                .help("Camera rotation speed"),
        )
        .arg(
            Arg::new("camera-zoom-speed")
                .long("camera-zoom-speed")
                .value_parser(value_parser!(f32))
                .help("Camera zooming speed"),
        )
        .arg(
            Arg::new("camera-pos")
                .long("camera-pos")
                .short('p')
                .value_parser(parse_vector3)
                .help("Initial camera position 'x,y,z'"),
        )
        .arg(
            Arg::new("camera-target")
                .long("camera-target")
                .short('t')
                .value_parser(parse_vector3)
                .help("Initial camera target 'x,y,z'"),
        )
        .arg(
            Arg::new("light-ambient")
                .long("light-ambient")
                .value_parser(value_parser!(f32))
                .help("Ambient lighting strength (0.0 - 1.0)"),
        )
        .arg(
            Arg::new("light-diffuse")
                .long("light-diffuse")
                .value_parser(value_parser!(f32))
                .help("Diffuse lighting strength (0.0 - 1.0)"),
        )
        .arg(
            Arg::new("light-specular")
                .long("light-specular")
                .value_parser(value_parser!(f32))
                .help("Specular lighting strength (0.0 - 1.0)"),
        )
        .arg(
            Arg::new("light-shininess")
                .long("light-shininess")
                .value_parser(value_parser!(u32))
                .help("Specular shininess exponent"),
        )
        .arg(
            Arg::new("fov")
                .long("fov")
                .value_parser(value_parser!(f32))
                .help("Field Of View in degrees"),
        )
        .arg(
            Arg::new("near")
                .long("near")
                .value_parser(value_parser!(f32))
                .help("Near frustum face"),
        )
        .arg(
            Arg::new("far")
                .long("far")
                .value_parser(value_parser!(f32))
                .help("Far frustum face"),
        )
        .arg(
            Arg::new("max-fps")
                .long("max-fps")
                .short('f')
                .value_parser(value_parser!(u32))
                .help("Maximum FPS"),
        )
        .arg(
            Arg::new("show-fps")
                .long("show-fps")
                .help("Show FPS")
                .action(ArgAction::SetTrue),
        )
}

fn parse_vector3(s: &str) -> Result<Vector3, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err("Expected format: 'x,y,z'".to_string());
    }

    let x = parts[0]
        .parse::<f32>()
        .map_err(|e| format!("Invalid X coordinate: {}", e))?;
    let y = parts[1]
        .parse::<f32>()
        .map_err(|e| format!("Invalid Y coordinate: {}", e))?;
    let z = parts[2]
        .parse::<f32>()
        .map_err(|e| format!("Invalid Z coordinate: {}", e))?;

    Ok(Vector3::new(x, y, z))
}
