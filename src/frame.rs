use num_complex::Complex64;

pub struct MandelbrotFrame {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub iter_max: u32,
    pub width: u32,
    pub height: u32,
}

impl MandelbrotFrame {
    pub fn interpolated_x(&self, px: u32) -> f64 {
        self.xmin + (px as f64) * self.xrange() / (self.width as f64)
    }

    pub fn xrange(&self) -> f64 {
        self.xmax - self.xmin
    }
    pub fn interpolated_y(&self, py: u32) -> f64 {
        self.ymin + (py as f64) * self.yrange() / (self.height as f64)
    }

    pub fn interpolated(&self, px: u32, py: u32) -> Complex64 {
        Complex64::new(self.interpolated_x(px), self.interpolated_y(py))
    }

    pub fn yrange(&self) -> f64 {
        self.ymax - self.ymin
    }

    pub fn new(x: u32, y: u32) -> MandelbrotFrame {
        MandelbrotFrame {
            width: x,
            height: y,
            xmin: -2.2,
            xmax: 0.6,
            ymin: -1.5,
            ymax: 1.5,
            iter_max: 500,
        }
    }

    pub fn iterations(&self, zp: Complex64) -> u32 {
        let mut z = zp;
        let mut iteration: u32 = 0;
        while z.norm() <= 4.0 && iteration < self.iter_max {
            z = z * z + zp;
            iteration = iteration + 1
        }
        return iteration;
    }

    pub fn compute<E>(
        &self,
        mut f: impl FnMut(u32, u32, u32) -> Result<(), E>,
    ) -> Result<(), E> {
        for x in 0..self.width {
            let sx = self.interpolated_x(x);
            for y in 0..self.height {
                let sy = self.interpolated_y(y);
                let z = Complex64::new(sx, sy);
                f(x, y, self.iterations(z))?;
            }
        }
        Ok(())
    }
}
