const MIN_PITCH: f32 = -89.0;
const MAX_PITCH: f32 = 89.0;

pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    close: bool,
    yaw: f32,
    pitch: f32,
    fov: f32,
}

impl Input {
    pub fn new() -> Self {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            close: false,
            yaw: -90.0,
            pitch: 0.0,
            fov: 45.0,
        }
    }

    pub fn up(&self) -> bool {
        self.up
    }

    pub fn set_up(&mut self, pressed: bool) {
        self.up = pressed;
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn set_down(&mut self, pressed: bool) {
        self.down = pressed;
    }

    pub fn left(&self) -> bool {
        self.left
    }

    pub fn set_left(&mut self, pressed: bool) {
        self.left = pressed;
    }

    pub fn right(&self) -> bool {
        self.right
    }

    pub fn set_right(&mut self, pressed: bool) {
        self.right = pressed;
    }

    pub fn close(&self) -> bool {
        self.close
    }

    pub fn set_close(&mut self, pressed: bool) {
        self.close = pressed;
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn set_yaw(&mut self, value: f32) {
        self.yaw = value;
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn set_pitch(&mut self, value: f32) {
        if value < MIN_PITCH {
            self.pitch = MIN_PITCH;
        } else if value > MAX_PITCH {
            self.pitch = MAX_PITCH;
        } else {
            self.pitch = value;
        }
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn set_fov(&mut self, value: f32) {
        self.fov = value;
    }

}