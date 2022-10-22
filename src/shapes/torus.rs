use crate::shapes::shape::Shape;
use std::f32::consts::TAU;

pub struct TorusPoints {
    psi: f32,
    theta: f32,
    small_r: f32,
    big_r: f32,
    x_rotation: f32,
    #[allow(dead_code)]
    y_rotation: f32,
    z_rotation: f32,
}

impl TorusPoints {
    pub fn new(small_r: f32, big_r: f32, shape: &Shape) -> Self {
        TorusPoints {
            small_r,
            big_r,
            x_rotation: shape.x_rotation,
            y_rotation: shape.y_rotation,
            z_rotation: shape.z_rotation,
            psi: 0.0,
            theta: 0.0,
        }
    }
}

impl Iterator for TorusPoints {
    type Item = [f32; 6];
    fn next(&mut self) -> Option<Self::Item> {
        static STEP: f32 = 0.05;
        let (sin_x_r, cos_x_r) = self.x_rotation.sin_cos();
        let (sin_z_r, cos_z_r) = self.z_rotation.sin_cos();
        while self.psi < TAU {
            while self.theta < TAU {
                self.theta += STEP;
                let (sin_t, cos_t) = self.theta.sin_cos();
                let (sin_p, cos_p) = self.psi.sin_cos();
                // x = (r*cos(the) + R)*cos(delta)*cos(psi) - r*sin(delta)*sin(the)
                let x = (self.small_r * cos_t + self.big_r) * cos_x_r * cos_p
                    - self.small_r * sin_x_r * sin_t;
                // y = ((r*cos(the) + R)*sin(delta)*cos(psi) + r*sin(the)*cos(delta))*sin(a) - (r*cos(the) + R)*sin(psi)*cos(a)
                let y = ((self.small_r * cos_t + self.big_r) * sin_x_r * cos_p
                    + self.small_r * sin_t * cos_x_r)
                    * sin_z_r
                    - (self.small_r * cos_t + self.big_r) * sin_p * cos_z_r;
                // z = ((r*cos(the) + R)*sin(delta)*cos(psi) + r*sin(the)*cos(delta))*cos(a) + (r*cos(the) + R)*sin(a)*sin(psi)
                let z = ((self.small_r * cos_t + self.big_r) * sin_x_r * cos_p
                    + self.small_r * sin_t * cos_x_r)
                    * cos_z_r
                    + (self.small_r * cos_t + self.big_r) * sin_z_r * sin_p;
                // x_normal = -sin(delta)*sin(the) + cos(delta)*cos(psi)*cos(the)
                let x_n = -sin_x_r * sin_t + cos_x_r * cos_p * cos_t;
                // y_normal = (sin(delta)*cos(psi)*cos(the) + sin(the)*cos(delta))*sin(a) - sin(psi)*cos(a)*cos(the)
                let y_n =
                    (sin_x_r * cos_p * cos_t + sin_t * cos_x_r) * sin_z_r - sin_p * cos_z_r * cos_t;
                // z_normal = (sin(delta)*cos(psi)*cos(the) + sin(the)*cos(delta))*cos(a) + sin(a)*sin(psi)*cos(the)
                let z_n =
                    (sin_x_r * cos_p * cos_t + sin_t * cos_x_r) * cos_z_r + sin_z_r * sin_p * cos_t;
                return Some([x, y, z, x_n, y_n, z_n]);
            }
            self.theta = 0.0;
            self.psi += STEP;
        }
        None
    }
}
