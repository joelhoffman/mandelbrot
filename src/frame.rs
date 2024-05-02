use array2d::Array2D;
pub struct MandelbrotFrame {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
    pub iter_max: u32,
    pub results: Array2D<u32>,
}

impl MandelbrotFrame {
    pub fn interpolated_x(&self, px: usize) -> f32 {
        self.xmin + (px as f32) * self.xrange() / (self.width() as f32)
    }

    pub fn xrange(&self) -> f32 {
        self.xmax - self.xmin
    }
    pub fn interpolated_y(&self, py: usize) -> f32 {
        self.ymin + (py as f32) * self.yrange() / (self.height() as f32)
    }

    pub fn yrange(&self) -> f32 {
        self.ymax - self.ymin
    }

    pub fn width(&self) -> usize {
        self.results.num_columns()
    }

    pub fn height(&self) -> usize {
        self.results.num_rows()
    }

    pub fn new(x: usize, y: usize) -> MandelbrotFrame {
        MandelbrotFrame {
            results: Array2D::filled_with(0, y, x),
            xmin: -2.2,
            xmax: 0.6,
            ymin: -1.5,
            ymax: 1.5,
            iter_max: 1000,
        }
    }

    pub fn iterations(&self, r: f32, i: f32) -> u32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut iteration: u32 = 0;
        while x * x + y * y <= 4.0 && iteration < self.iter_max {
            let xtemp = x * x - y * y + r;
            y = 2.0 * x * y + i;
            x = xtemp;
            iteration = iteration + 1
        }
        return iteration;
    }

    pub fn compute(&mut self) -> () {
        for x in 0..self.width() {
            let sx = self.interpolated_x(x);
            for y in 0..self.height() {
                let sy = self.interpolated_y(y);
                self.results.set(y, x, self.iterations(sx, sy)).unwrap();
            }
        }
    }
}
