#[derive(Clone, Debug)]
pub struct CropArea {
    start_x: i64,
    start_y: i64,
    end_x: i64,
    end_y: i64,
}

impl CropArea {
    pub fn new() -> Self {
        Self {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
        }
    }

    pub fn new_with_params(start_x: i64, start_y: i64, end_x: i64, end_y: i64) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }

    pub fn set_start_x(&mut self, x: i64) {
        self.start_x = x;
    }
    pub fn set_start_y(&mut self, y: i64) {
        self.start_y = y;
    }
    pub fn set_end_x(&mut self, x: i64) {
        self.end_x = x;
    }
    pub fn set_end_y(&mut self, y: i64) {
        self.end_y = y;
    }

    pub fn set_start(&mut self, x: i64, y: i64) {
        self.start_x = x;
        self.start_y = y;
    }

    pub fn set_end(&mut self, x: i64, y: i64) {
        self.end_x = x;
        self.end_y = y;
    }

    pub fn get_start_x(&self) -> i64 {
        self.start_x
    }
    pub fn get_start_y(&self) -> i64 {
        self.start_y
    }
    pub fn get_end_x(&self) -> i64 {
        self.end_x
    }
    pub fn get_end_y(&self) -> i64 {
        self.end_y
    }

    pub fn get_start(&self) -> (i64, i64) {
        (self.start_x, self.start_y)
    }

    pub fn get_end(&self) -> (i64, i64) {
        (self.end_x, self.end_y)
    }

    pub fn get_width(&self) -> i64 {
        self.end_x - self.start_x
    }

    pub fn get_height(&self) -> i64 {
        self.end_y - self.start_y
    }
}

#[derive(Clone, Debug)]
pub struct ImageOffset {
    x: i64,
    y: i64,
    aspect_ratio: f64,
}

impl ImageOffset {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            aspect_ratio: 0.0,
        }
    }

    pub fn new_with_params(x: i64, y: i64, aspect_ratio: f64) -> Self {
        Self { x, y, aspect_ratio }
    }

    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }
    pub fn get_y(&self) -> i64 {
        self.y
    }
    pub fn get_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
}
