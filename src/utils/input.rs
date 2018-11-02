pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    close: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            close: false,
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

}