pub struct Lines {
    pub outer_width: f32,
    pub box_width: f32,
    pub normal_width: f32,
}

impl Default for Lines {
    fn default() -> Self {
        Lines {
            outer_width: 5.0,
            box_width: 4.0,
            normal_width: 2.0,
        }
    }
}
