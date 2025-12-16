use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Print;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{ExecutableCommand, QueueableCommand, event, terminal};
use std::error::Error;
use std::io::{Write, stdout};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use crate::camera::look_at_camera::LookAtCamera;
use crate::config::{Config, ShadingMode};
use crate::geometry::mesh::Mesh;
use crate::io::obj_loader::ObjLoader;
use crate::output::brailler_formatter::BrailleColorFormatter;
use crate::rendering::renderer::Renderer;

const MESH_MAX_EXTENT: f32 = 2.0;
const FPS_MAX_SAMPLES: u32 = 10;

pub struct App {
    config: Config,
    renderer: Renderer,
    camera: LookAtCamera,
    mesh: Mesh,
    output: BrailleColorFormatter,
    fps_counter: FpsCounter,

    is_running: bool,
}

impl App {
    pub fn new<P: AsRef<Path>>(obj_file: P, config: Config) -> Self {
        let renderer = Renderer::new(&config);

        let raw_mesh = ObjLoader::load_from_file(obj_file)
            .unwrap_or_else(|e| panic!("failed to load model: {:?}", e));

        let mut mesh = match config.shading_mode {
            ShadingMode::Flat => Mesh::with_flat_normals(raw_mesh)
                .unwrap_or_else(|e| panic!("failed to create mesh: {:?}", e)),
            ShadingMode::Smooth => Mesh::with_smooth_normals(raw_mesh)
                .unwrap_or_else(|e| panic!("failed to create mesh: {:?}", e)),
        };
        mesh.fit(MESH_MAX_EXTENT);
        mesh.centering();

        let aspect = config.frame_width as f32 / config.frame_height as f32;
        let camera = LookAtCamera::new(
            config.camera_pos,
            config.camera_target,
            (aspect, 1.0),
            config.fov.to_radians(),
            config.near,
            config.far,
        );

        App {
            config,
            renderer,
            camera,
            mesh,
            output: BrailleColorFormatter,
            fps_counter: FpsCounter::new(FPS_MAX_SAMPLES),
            is_running: true,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        if self.config.static_mode {
            self.run_snapshot()
        } else {
            self.run_interactive()
        }
    }

    fn run_snapshot(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();
        self.render(&mut stdout)
    }

    fn run_interactive(&mut self) -> Result<(), Box<dyn Error>> {
        terminal::enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        self.main_loop()?;

        stdout().execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }

    fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = stdout();
        let frame_duration = Duration::from_secs_f32(1.0 / self.config.max_fps as f32);

        let mut frame_start = Instant::now() - frame_duration;

        while self.is_running {
            let dt = frame_start.elapsed().as_secs_f32();
            self.fps_counter.tick(dt);
            self.handle_input(dt)?;

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
                        self.zoom_in(dt);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        self.zoom_out(dt);
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        self.look_left(dt);
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        self.look_right(dt);
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.look_up(dt);
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        self.look_down(dt);
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') | KeyCode::Esc => {
                        self.is_running = false;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn Error>> {
        self.renderer.render(&self.mesh, &self.camera);
        let frame = self.renderer.frame(&self.output);
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Print(frame))?;
        if self.config.show_fps {
            stdout.queue(Print(format!("\rFPS={:.1}", self.fps_counter.fps())))?;
        }
        stdout.flush()?;
        Ok(())
    }

    fn look_up(&mut self, dt: f32) {
        self.camera
            .orbit_around_target(0.0, self.config.camera_rotation_speed * dt);
    }

    fn look_down(&mut self, dt: f32) {
        self.camera
            .orbit_around_target(0.0, -self.config.camera_rotation_speed * dt);
    }

    fn look_left(&mut self, dt: f32) {
        self.camera
            .orbit_around_target(self.config.camera_rotation_speed * dt, 0.0);
    }

    fn look_right(&mut self, dt: f32) {
        self.camera
            .orbit_around_target(-self.config.camera_rotation_speed * dt, 0.0);
    }

    fn zoom_in(&mut self, dt: f32) {
        self.camera.zoom(-self.config.camera_zoom_speed * dt);
    }

    fn zoom_out(&mut self, dt: f32) {
        self.camera.zoom(self.config.camera_zoom_speed * dt);
    }
}

struct FpsCounter {
    samples: u32,
    i: u32,
    duration: Duration,
    fps: f32,
}

impl FpsCounter {
    fn new(samples: u32) -> Self {
        Self {
            samples,
            i: 0,
            duration: Duration::from_secs(0),
            fps: 0.0,
        }
    }

    fn tick(&mut self, dt: f32) {
        self.i += 1;
        self.duration += Duration::from_secs_f32(dt);
        if self.i >= self.samples {
            self.fps = self.i as f32 / self.duration.as_secs_f32();
            self.i = 0;
            self.duration = Duration::from_secs(0);
        }
    }

    fn fps(&self) -> f32 {
        self.fps
    }
}
