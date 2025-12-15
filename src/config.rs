use crate::math::vectors::Vector3;

#[derive(clap::ValueEnum, Clone, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum ShadingMode {
    Flat,
    Smooth,
}

#[derive(Debug)]
pub struct Config {
    pub static_mode: bool,

    pub frame_width: usize,
    pub frame_height: usize,

    pub backface_culling: bool,
    pub shading_mode: ShadingMode,

    pub camera_speed: f32,
    pub camera_rotation_speed: f32,
    pub camera_zoom_speed: f32,
    pub camera_pos: Vector3,
    pub camera_target: Vector3,

    pub light_ambient: f32,
    pub light_diffuse: f32,
    pub light_specular: f32,
    pub light_shininess: u32,

    pub fov: f32,
    pub near: f32,
    pub far: f32,

    pub max_fps: u32,

    pub show_fps: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            static_mode: false,
            frame_width: 80,
            frame_height: 24, // Стандартный размер терминала
            backface_culling: true,
            shading_mode: ShadingMode::Smooth,
            camera_speed: 2.0,
            camera_rotation_speed: 90.0,
            camera_zoom_speed: 2.0,
            camera_pos: Vector3::new(0.0, 0.0, 2.0),
            camera_target: Vector3::new(0.0, 0.0, 0.0),
            light_ambient: 0.05,
            light_diffuse: 0.7,
            light_specular: 0.25,
            light_shininess: 8,
            fov: 60.0,
            near: 0.1,
            far: 5.0,
            max_fps: 60,
            show_fps: false,
        }
    }
}

impl Config {
    pub fn with_clap_matches(mut self, matches: &clap::ArgMatches) -> Self {
        if matches.get_flag("static-mode") {
            self.static_mode = true;
        }
        if let Some(&width) = matches.get_one::<usize>("frame-width") {
            self.frame_width = width * 2;
        }
        if let Some(&height) = matches.get_one::<usize>("frame-height") {
            self.frame_height = height * 4;
        }
        if matches.get_flag("no-culling") {
            self.backface_culling = false;
        }
        if let Some(mode) = matches.get_one::<ShadingMode>("shading") {
            self.shading_mode = mode.clone();
        }
        if let Some(&camera_speed) = matches.get_one::<f32>("camera-speed") {
            self.camera_speed = camera_speed;
        }
        if let Some(&camera_rotation_speed) = matches.get_one::<f32>("camera-rotation-speed") {
            self.camera_rotation_speed = camera_rotation_speed;
        }
        if let Some(&camera_zooming_speed) = matches.get_one::<f32>("camera-zoom-speed") {
            self.camera_zoom_speed = camera_zooming_speed;
        }
        if let Some(&camera_pos) = matches.get_one::<Vector3>("camera-pos") {
            self.camera_pos = camera_pos;
        }
        if let Some(&camera_target) = matches.get_one::<Vector3>("camera-target") {
            self.camera_target = camera_target;
        }
        if let Some(&light_ambient) = matches.get_one::<f32>("light-ambient") {
            self.light_ambient = light_ambient;
        }
        if let Some(&light_diffuse) = matches.get_one::<f32>("light-diffuse") {
            self.light_diffuse = light_diffuse;
        }
        if let Some(&light_specular) = matches.get_one::<f32>("light-specular") {
            self.light_specular = light_specular;
        }
        if let Some(&light_shininess) = matches.get_one::<u32>("light-shininess") {
            self.light_shininess = light_shininess;
        }
        if let Some(&fov) = matches.get_one::<f32>("fov") {
            self.fov = fov;
        }
        if let Some(&near) = matches.get_one::<f32>("near") {
            self.near = near;
        }
        if let Some(&far) = matches.get_one::<f32>("far") {
            self.far = far;
        }
        if let Some(&max_fps) = matches.get_one::<u32>("max-fps") {
            self.max_fps = max_fps;
        }
        if matches.get_flag("show-fps") {
            self.show_fps = true;
        }
        self
    }

    pub fn with_resolution(mut self, width: usize, height: usize) -> Self {
        self.frame_width = width;
        self.frame_height = height;
        self
    }
}
