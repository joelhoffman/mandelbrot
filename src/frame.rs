use stopwatch::Stopwatch;

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

    pub fn interpolated(&self, px: u32, py: u32) -> (f64,f64) {
        (self.interpolated_x(px), self.interpolated_y(py))
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
            iter_max: 1000,
        }
    }

    pub fn iterations(&self, sx: f64, sy: f64) -> u32 {
        let mut zx = sx;
        let mut zy = sy;
        let mut iteration: u32 = 0;
        while zx*zx+zy*zy <= 4.0 && iteration < self.iter_max {
            let temp_x = zx;
            zx = zx*zx-zy*zy+sx;
            zy = 2.0*temp_x*zy+sy;

            iteration = iteration + 1
        }
        return iteration;
    }

    pub fn compute<E>(
        &self,
        mut f: impl FnMut(u32, u32, u32) -> Result<(), E>,
    ) -> Result<(), E> {
        let sw = Stopwatch::start_new();
        let mut f_sw = Stopwatch::new();
        for x in 0..self.width {
            let sx = self.interpolated_x(x);
            for y in 0..self.height {
                let sy = self.interpolated_y(y);
                // let z = Complex64::new(sx, sy);
                let iter = self.iterations(sx,sy);
                f_sw.start();
                f(x, y, iter)?;
                f_sw.stop();
            }
        }
        println!("Compute took {:.6}s of which {:.6}s is in the closure", sw.elapsed().as_secs_f32(), f_sw.elapsed().as_secs_f32());
        Ok(())
    }
}
