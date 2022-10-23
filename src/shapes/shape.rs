pub struct Shape {
    pub height: usize,
    pub width: usize,
    pub viewer_depth: f32,
    pub x_rotation: f32,
    pub y_rotation: f32,
    pub z_rotation: f32,
    pub light_source: [f32; 3],
    pub offset_height: f32,
    pub offset_width: f32,
    pub offset_depth: f32,
}
impl Shape {
    pub fn new(height: usize, width: usize, viewer_depth: f32) -> Self {
        Shape {
            height,
            width,
            viewer_depth,
            light_source: [0.0, 0.0, 0.0],
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            offset_height: 0.0,
            offset_width: 0.0,
            offset_depth: 0.0,
        }
    }
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.x_rotation += x;
        self.y_rotation += y;
        self.z_rotation += z;
    }
    pub fn shift(&mut self, height: f32, width: f32, depth: f32) {
        self.offset_height += height;
        self.offset_width += width;
        self.offset_depth += depth;
    }
    pub fn shift_light_source(&mut self, x: f32, y: f32, z: f32) {
        let tmp = [
            self.light_source[0] + x,
            self.light_source[1] + y,
            self.light_source[2] + z,
        ];
        let modulus = (tmp[0] * tmp[0] + tmp[1] * tmp[1] + tmp[2] * tmp[2]).sqrt();
        self.light_source = [tmp[0] / modulus, tmp[1] / modulus, tmp[2] / modulus];
    }

    pub fn generate_lumi(&self, points: impl Iterator<Item = [f32; 6]>) -> Vec<Vec<f32>> {
        let mut z_buf = vec![vec![-f32::MAX; self.height]; self.width];
        let mut buf = vec![vec![-f32::MAX; self.height]; self.width];
        for [x, y, z, x_n, y_n, z_n] in points {
            let x_pixel = (x * -self.viewer_depth / (self.offset_depth - self.viewer_depth - z)
                + self.offset_width) as usize;
            let y_pixel = (y * -self.viewer_depth / (self.offset_depth - self.viewer_depth - z)
                + self.offset_height) as usize;
            if x_pixel < self.width && y_pixel < self.height && z > z_buf[x_pixel][y_pixel] {
                z_buf[x_pixel][y_pixel] = z;
                let normal = [x_n, y_n, z_n];
                buf[x_pixel][y_pixel] = Shape::dot_product(&normal, &self.light_source);
            }
        }
        buf
    }
    fn dot_product(a: &[f32; 3], b: &[f32; 3]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
}
