use crate::shapes::shape::Shape;
use std::f32::consts::TAU;

pub struct SpherePoints {
    psi: f32,
    theta: f32,
    r: f32,
}

impl SpherePoints {
    #[allow(unused_variables)]
    pub fn new(r: f32, shape: &Shape) -> Self {
        SpherePoints {
            r,
            psi: 0.0,
            theta: 0.0,
        }
    }
}

impl Iterator for SpherePoints {
    type Item = [f32; 6];
    fn next(&mut self) -> Option<Self::Item> {
        static STEP: f32 = 0.05;
        while self.psi < TAU {
            while self.theta < TAU {
                self.theta += STEP;
                let (sin_t, cos_t) = self.theta.sin_cos();
                let (sin_p, cos_p) = self.psi.sin_cos();
                // x = r*cos(t)*sin(p)
                let x = self.r * cos_t * sin_p;
                // y = r*sin(t)*sin(p)
                let y = self.r * sin_t * sin_p;
                // z = r*cos(p)
                let z = self.r * cos_p;
                // x_normal = -sin(delta)*sin(the) + cos(delta)*cos(psi)*cos(the)
                let x_n = cos_t * sin_p;
                // y_normal = (sin(delta)*cos(psi)*cos(the) + sin(the)*cos(delta))*sin(a) - sin(psi)*cos(a)*cos(the)
                let y_n = sin_t * sin_p;
                // z_normal = (sin(delta)*cos(psi)*cos(the) + sin(the)*cos(delta))*cos(a) + sin(a)*sin(psi)*cos(the)
                let z_n = cos_p;
                return Some([x, y, z, x_n, y_n, z_n]);
            }
            self.theta = 0.0;
            self.psi += STEP;
        }
        None
    }
}
