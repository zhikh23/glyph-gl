pub mod camera;
pub mod geometry;
pub mod io;
pub mod math;
pub mod output;
pub mod rendering;

use clap::{Arg, Command};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{ExecutableCommand, QueueableCommand, event, terminal};
use std::error::Error;
use std::io::{Write, stdout};
use std::ops::Sub;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use crate::camera::look_at_camera::LookAtCamera;
use crate::geometry::mesh::Mesh;
use crate::io::obj_loader::ObjLoader;
use crate::math::vectors::Vector3;
use crate::output::brailler_formatter::BrailleColorFormatter;
use crate::rendering::renderer::Renderer;

const ROTATE_SPEED_RADS: f32 = 90.0;
const MOVEMENT_SPEED: f32 = 2.0;

pub struct App {
    renderer: Renderer,
    is_running: bool,
    rotate_speed: f32,

    camera: LookAtCamera,
    mesh: Mesh,
}

impl App {
    pub fn new<P: AsRef<Path>>(obj_file: P) -> Self {
        let tmp = terminal::size().unwrap_or((80, 24));
        let mut term_size = (tmp.0 as usize, tmp.1 as usize);
        term_size.1 -= 3;
        let aspect = 0.5 * term_size.0 as f32 / term_size.1 as f32;

        let output = Box::new(BrailleColorFormatter);
        let renderer = Renderer::new(term_size, output);

        let raw_mesh = ObjLoader::load_from_file(obj_file)
            .unwrap_or_else(|e| panic!("failed to load model: {:?}", e));
        let mut mesh = Mesh::with_smooth_normals(raw_mesh)
            .unwrap_or_else(|e| panic!("failed to create mesh: {:?}", e));
        mesh.fit(2.0);

        let mesh_center = mesh.center();
        let camera = LookAtCamera::new(
            Vector3::new(0.0, mesh_center.y, 2.0),
            mesh_center,
            (aspect * 2.0, 2.0),
        );

        App {
            renderer,
            is_running: true,
            rotate_speed: ROTATE_SPEED_RADS,
            camera,
            mesh,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        terminal::enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        self.main_loop()?;

        stdout().execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }

    fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();
        let frame_duration = Duration::from_millis(16); // ~60 FPS

        let mut frame_start = Instant::now().sub(frame_duration);

        let mut i = 0;
        let mut fps: f32 = 60.0;
        let mut acc: f32 = 0.0;

        while self.is_running {
            let dt = frame_start.elapsed().as_secs_f32();
            self.handle_input(dt)?;
            eprintln!("FPS={:.1}", fps);
            acc += dt;
            i += 1;
            if i >= 10 {
                i = 0;
                fps = 10.0 / acc;
                acc = 0.0;
            }
            frame_start = Instant::now();

            self.render(&mut stdout)?;

            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                thread::sleep(frame_duration - elapsed);
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, dt: f32) -> Result<(), Box<dyn Error>> {
        while event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: _,
                state: _,
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        self.look_up(dt);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        self.look_down(dt);
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        self.look_left(dt);
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        self.look_right(dt);
                    }

                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.zoom_in(dt);
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        self.zoom_out(dt);
                    }

                    // Выход
                    KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Esc => {
                        self.is_running = false;
                    }

                    // Отпускание клавиш
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn Error>> {
        self.renderer.render(&self.mesh, &self.camera);
        let frame = self.renderer.frame();
        let frame = frame.replace("\n", "\r\n");
        stdout.queue(MoveTo(0, 0))?;
        println!(
            "eye={:?} target={:?}\r",
            self.camera.eye(),
            self.camera.target()
        );
        stdout.queue(Print(frame))?;
        stdout.flush()?;
        Ok(())
    }

    fn look_up(&mut self, dt: f32) {
        self.camera.orbit_around_target(0.0, self.rotate_speed * dt);
    }

    fn look_down(&mut self, dt: f32) {
        self.camera
            .orbit_around_target(0.0, -self.rotate_speed * dt);
    }

    fn look_left(&mut self, dt: f32) {
        self.camera.orbit_around_target(self.rotate_speed * dt, 0.0);
    }

    fn look_right(&mut self, dt: f32) {
        self.camera
            .orbit_around_target(-self.rotate_speed * dt, 0.0);
    }

    fn zoom_in(&mut self, dt: f32) {
        self.camera.zoom(-MOVEMENT_SPEED * dt);
    }

    fn zoom_out(&mut self, dt: f32) {
        self.camera.zoom(MOVEMENT_SPEED * dt);
    }
}

fn main() {
    let matches = Command::new("GlyphGL")
        .version("0.9")
        .author("Kirill Zhikharev")
        .about("Subpixel terminal 3D .obj render")
        .arg(
            Arg::new("file")
                .help("Wavefront OBJ file")
                .required(true)
                .index(1),
        )
        .get_matches();
    let input_path = matches.get_one::<String>("file").unwrap();
    let mut app = App::new(input_path);
    app.run().unwrap();
}
