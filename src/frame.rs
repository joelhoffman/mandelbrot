use num_complex::Complex32;

pub struct MandelbrotFrame {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
    pub iter_max: u32,
    pub width: usize,
    pub height: usize,
}

impl MandelbrotFrame {
    pub fn interpolated_x(&self, px: usize) -> f32 {
        self.xmin + (px as f32) * self.xrange() / (self.width as f32)
    }

    pub fn xrange(&self) -> f32 {
        self.xmax - self.xmin
    }
    pub fn interpolated_y(&self, py: usize) -> f32 {
        self.ymin + (py as f32) * self.yrange() / (self.height as f32)
    }

    pub fn interpolated(&self, px: usize, py: usize) -> Complex32 {
        Complex32::new(self.interpolated_x(px), self.interpolated_y(py))
    }

    pub fn yrange(&self) -> f32 {
        self.ymax - self.ymin
    }

    pub fn new(x: usize, y: usize) -> MandelbrotFrame {
        MandelbrotFrame {
            width: x,
            height: y,
            xmin: -2.2,
            xmax: 0.6,
            ymin: -1.5,
            ymax: 1.5,
            iter_max: 200,
        }
    }

    pub fn iterations(&self, zp: Complex32) -> u32 {
        let mut z = zp;
        let mut iteration: u32 = 0;
        while z.norm() <= 4.0 && iteration < self.iter_max {
            z = z * z + zp;
            iteration = iteration + 1
        }
        return iteration;
    }

    pub fn compute<E>(
        &mut self,
        mut f: impl FnMut(usize, usize, u32) -> Result<(), E>,
    ) -> Result<(), E> {
        for x in 0..self.width {
            let sx = self.interpolated_x(x);
            for y in 0..self.height {
                let sy = self.interpolated_y(y);
                let z = Complex32::new(sx, sy);
                f(x, y, self.iterations(z))?;
            }
        }
        Ok(())
    }
}
