#[derive(Clone, Debug)]
pub struct CropArea {
    pub start_x: i64,
    pub start_y: i64,
    pub end_x: i64,
    pub end_y: i64,
}

#[derive(Clone, Debug)]
pub struct ImageOffset {
    pub x: i64,
    pub y: i64,
    pub aspect_ratio: f64,
}